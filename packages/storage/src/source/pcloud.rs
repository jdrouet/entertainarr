use futures::StreamExt;
use pcloud::{
    credentials::Credentials,
    folder::list::FolderListCommand,
    general::userinfo::UserInfoCommand,
    http::{HttpClient, HttpClientBuilder},
    prelude::HttpCommand,
    region::Region,
};
use reqwest::header::RANGE;
use std::{ops::Bound, path::PathBuf, sync::Arc};
use tokio::io::AsyncRead;
use tokio_util::io::StreamReader;

impl From<pcloud::entry::Entry> for crate::entry::EntryInfo {
    fn from(value: pcloud::entry::Entry) -> Self {
        match value {
            pcloud::entry::Entry::Folder(folder) => {
                crate::entry::EntryInfo::Directory(folder.into())
            }
            pcloud::entry::Entry::File(file) => crate::entry::EntryInfo::File(file.into()),
        }
    }
}

impl From<pcloud::entry::Folder> for crate::entry::DirectoryInfo {
    fn from(value: pcloud::entry::Folder) -> Self {
        crate::entry::DirectoryInfo {
            name: value.base.name,
            created_at: value.base.created.timestamp() as u64,
        }
    }
}

impl From<pcloud::entry::File> for crate::entry::FileInfo {
    fn from(value: pcloud::entry::File) -> Self {
        crate::entry::FileInfo {
            name: value.base.name,
            size: value.size.unwrap_or(0) as u64,
            created_at: value.base.created.timestamp() as u64,
            modified_at: value.base.modified.timestamp() as u64,
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub region: String,
    pub username: String,
    pub password: String,
    pub path: PathBuf,
}

impl Config {
    pub fn build(&self) -> std::io::Result<Source> {
        let client = HttpClientBuilder::default()
            .region(match self.region.as_str() {
                "eu" => Region::eu(),
                "us" => Region::us(),
                other => Region::new(other.to_string()),
            })
            .credentials(Credentials::UserPassword {
                username: self.username.clone(),
                password: self.password.clone(),
            })
            .build()
            .map_err(|err| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("unable to build http client: {err:?}"),
                )
            })?;
        Ok(Source(Arc::new(InnerSource {
            client,
            root: self.path.clone(),
        })))
    }
}

struct InnerSource {
    client: HttpClient,
    root: PathBuf,
}

impl std::fmt::Debug for InnerSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(Source))
            .field("client", &"[REDACTED]")
            .field("root", &self.root)
            .finish()
    }
}

#[derive(Clone, Debug)]
pub struct Source(Arc<InnerSource>);

impl AsRef<InnerSource> for Source {
    fn as_ref(&self) -> &InnerSource {
        &self.0
    }
}

impl crate::source::prelude::Source for Source {
    type File = PCloudFile;

    async fn healthcheck(&self) -> std::io::Result<()> {
        UserInfoCommand::new(false, false)
            .execute(&self.as_ref().client)
            .await
            .map(|_| ())
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::ConnectionRefused, err))
    }

    async fn list(&self, path: &str) -> std::io::Result<Vec<crate::entry::EntryInfo>> {
        let real_path = if path.is_empty() {
            self.as_ref().root.clone()
        } else {
            self.as_ref().root.join(path)
        };
        let folder = FolderListCommand::new(real_path.to_string_lossy().to_string().into())
            .recursive(false)
            .execute(&self.as_ref().client)
            .await
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::ConnectionRefused, err))?;
        let Some(list) = folder.contents else {
            return Ok(Vec::new());
        };
        Ok(list.into_iter().map(|item| item.into()).collect())
    }

    async fn file(&self, path: &str) -> std::io::Result<Self::File> {
        let child = path.strip_prefix('/').unwrap_or(path);
        let real_path = self.as_ref().root.join(child).to_string_lossy().to_string();
        let checksum = pcloud::file::checksum::FileCheckSumCommand::new(real_path.into())
            .execute(&self.as_ref().client)
            .await
            .map_err(std::io::Error::other)?;
        let size = checksum
            .metadata
            .size
            .ok_or_else(|| std::io::Error::other("unable to read file size"))?
            as u64;
        Ok(PCloudFile {
            source: self.clone(),
            path: PathBuf::from(path),
            size,
        })
    }
}

fn range_header_value(range: (Bound<u64>, Bound<u64>)) -> String {
    match range {
        (Bound::Included(start), Bound::Included(end)) => format!("bytes={}-{}", start, end),
        (Bound::Included(start), Bound::Excluded(end)) => {
            format!("bytes={}-{}", start, end - 1)
        }
        (Bound::Included(start), Bound::Unbounded) => format!("bytes={}-", start),
        (Bound::Unbounded, Bound::Included(end)) => format!("bytes=-{}", end),
        (Bound::Unbounded, Bound::Excluded(end)) => format!("bytes=-{}", end - 1),
        (Bound::Unbounded, Bound::Unbounded) => format!("bytes=-"),
        (Bound::Excluded(start), Bound::Included(end)) => format!("bytes={}-{}", start + 1, end),
        (Bound::Excluded(start), Bound::Excluded(end)) => {
            format!("bytes={}-{}", start + 1, end - 1)
        }
        (Bound::Excluded(start), Bound::Unbounded) => format!("bytes={}-", start + 1),
    }
}

#[derive(Debug)]
pub struct PCloudFile {
    source: Source,
    path: PathBuf,
    size: u64,
}

impl crate::source::prelude::File for PCloudFile {
    type Reader = PCloudFileReader;

    fn path(&self) -> &std::path::Path {
        &self.path
    }

    fn size(&self) -> u64 {
        self.size
    }

    async fn reader(&self, range: (Bound<u64>, Bound<u64>)) -> std::io::Result<Self::Reader> {
        let path = self
            .source
            .0
            .root
            .join(&self.path)
            .to_string_lossy()
            .to_string();
        let link = pcloud::streaming::get_file_link::GetFileLinkCommand::new(path.into())
            .execute(&self.source.0.client)
            .await
            .map_err(std::io::Error::other)?;
        let client = reqwest::Client::new();
        let res = client
            .get(link)
            .header(RANGE, range_header_value(range))
            .send()
            .await
            .map_err(std::io::Error::other)?;

        let byte_stream = res.bytes_stream();

        // Convert reqwest::Error to std::io::Error
        let io_stream = byte_stream
            .map(|res| res.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)));

        Ok(PCloudFileReader {
            inner: Box::pin(StreamReader::new(io_stream)),
        })
    }
}

pub struct PCloudFileReader {
    inner: std::pin::Pin<Box<dyn AsyncRead + Send>>,
}

impl crate::source::prelude::FileReader for PCloudFileReader {}

impl AsyncRead for PCloudFileReader {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        self.get_mut().inner.as_mut().poll_read(cx, buf)
    }
}

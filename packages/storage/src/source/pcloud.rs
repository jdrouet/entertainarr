use std::path::PathBuf;

use pcloud::{
    credentials::Credentials,
    folder::list::FolderListCommand,
    general::userinfo::UserInfoCommand,
    http::{HttpClient, HttpClientBuilder},
    prelude::HttpCommand,
    region::Region,
};

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
        Ok(Source {
            client,
            root: self.path.clone(),
        })
    }
}

pub struct Source {
    client: HttpClient,
    root: PathBuf,
}

impl std::fmt::Debug for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(Source))
            .field("client", &"[REDACTED]")
            .field("root", &self.root)
            .finish()
    }
}

impl crate::source::prelude::Source for Source {
    async fn healthcheck(&self) -> std::io::Result<()> {
        UserInfoCommand::new(false, false)
            .execute(&self.client)
            .await
            .map(|_| ())
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::ConnectionRefused, err))
    }

    async fn list(&self, path: &str) -> std::io::Result<Vec<crate::entry::EntryInfo>> {
        let real_path = if path.is_empty() {
            self.root.clone()
        } else {
            self.root.join(path)
        };
        let folder = FolderListCommand::new(real_path.to_string_lossy().to_string().into())
            .recursive(false)
            .execute(&self.client)
            .await
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::ConnectionRefused, err))?;
        let Some(list) = folder.contents else {
            return Ok(Vec::new());
        };
        Ok(list.into_iter().map(|item| item.into()).collect())
    }
}

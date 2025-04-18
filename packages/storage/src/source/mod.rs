use std::{ops::Bound, pin::Pin};

use tokio::io::AsyncRead;

pub mod local;
pub mod pcloud;
pub mod prelude;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Config {
    Local(local::Config),
    #[serde(rename = "pcloud")]
    PCloud(pcloud::Config),
}

impl Config {
    pub fn build(&self) -> std::io::Result<AnySource> {
        match self {
            Self::Local(inner) => Ok(AnySource::Local(inner.build()?)),
            Self::PCloud(inner) => Ok(AnySource::PCloud(inner.build()?)),
        }
    }
}

#[derive(Clone, Debug)]
pub enum AnySource {
    Local(local::Source),
    PCloud(pcloud::Source),
}

impl crate::source::prelude::Source for AnySource {
    type File = AnyFile;

    async fn healthcheck(&self) -> std::io::Result<()> {
        match self {
            Self::Local(inner) => inner.healthcheck().await,
            Self::PCloud(inner) => inner.healthcheck().await,
        }
    }

    async fn list(&self, path: &str) -> std::io::Result<Vec<crate::entry::EntryInfo>> {
        match self {
            Self::Local(inner) => inner.list(path).await,
            Self::PCloud(inner) => inner.list(path).await,
        }
    }

    async fn file(&self, path: &str) -> std::io::Result<Self::File> {
        match self {
            Self::Local(inner) => inner.file(path).await.map(AnyFile::Local),
            Self::PCloud(inner) => inner.file(path).await.map(AnyFile::PCloud),
        }
    }
}

pub enum AnyFile {
    Local(crate::source::local::LocalFile),
    PCloud(crate::source::pcloud::PCloudFile),
}

impl crate::source::prelude::File for AnyFile {
    type Reader = AnyFileReader;

    fn path(&self) -> &std::path::Path {
        match self {
            Self::Local(inner) => inner.path(),
            Self::PCloud(inner) => inner.path(),
        }
    }

    fn size(&self) -> u64 {
        match self {
            Self::Local(inner) => inner.size(),
            Self::PCloud(inner) => inner.size(),
        }
    }

    async fn reader(&self, range: (Bound<u64>, Bound<u64>)) -> std::io::Result<Self::Reader> {
        match self {
            Self::Local(inner) => inner.reader(range).await.map(AnyFileReader::Local),
            Self::PCloud(inner) => inner.reader(range).await.map(AnyFileReader::PCloud),
        }
    }
}

pub enum AnyFileReader {
    Local(crate::source::local::LocalFileReader),
    PCloud(crate::source::pcloud::PCloudFileReader),
}

impl crate::source::prelude::FileReader for AnyFileReader {}

impl AsyncRead for AnyFileReader {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        match self.get_mut() {
            AnyFileReader::Local(reader) => Pin::new(reader).poll_read(cx, buf),
            AnyFileReader::PCloud(reader) => Pin::new(reader).poll_read(cx, buf),
        }
    }
}

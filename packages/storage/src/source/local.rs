use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::SystemTime,
};

use crate::entry::{DirectoryInfo, EntryInfo, FileInfo};

fn into_secs(time: SystemTime) -> u64 {
    time.duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub path: PathBuf,
}

impl Config {
    pub fn build(&self) -> std::io::Result<Source> {
        Ok(Source::new(self.path.clone()))
    }
}

impl From<Source> for super::AnySource {
    fn from(value: Source) -> Self {
        Self::Local(value)
    }
}

#[derive(Debug)]
struct InnerSource {
    root: PathBuf,
}

#[derive(Clone, Debug)]
pub struct Source(Arc<InnerSource>);

impl AsRef<InnerSource> for Source {
    fn as_ref(&self) -> &InnerSource {
        &self.0
    }
}

impl Source {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self(Arc::new(InnerSource { root: root.into() }))
    }
}

impl crate::source::prelude::Source for Source {
    type File = LocalFile;

    async fn healthcheck(&self) -> std::io::Result<()> {
        if self.as_ref().root.exists() {
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "root directory not found",
            ))
        }
    }

    async fn list(&self, path: &str) -> std::io::Result<Vec<crate::entry::EntryInfo>> {
        let path = self.as_ref().root.join(path);
        let mut dir = tokio::fs::read_dir(&path).await?;
        let mut res = Vec::new();

        while let Ok(Some(entry)) = dir.next_entry().await {
            let name: String = entry.file_name().to_string_lossy().to_string();
            let meta = entry.metadata().await?;
            let created_at = into_secs(meta.created()?);
            if meta.is_file() {
                let size = meta.len();
                res.push(EntryInfo::File(FileInfo {
                    name,
                    size,
                    created_at,
                    modified_at: into_secs(meta.modified()?),
                }));
            } else if meta.is_dir() {
                res.push(EntryInfo::Directory(DirectoryInfo { name, created_at }));
            }
        }
        Ok(res)
    }

    async fn file(&self, path: &str) -> std::io::Result<Self::File> {
        let child = path.strip_prefix('/').unwrap_or(path);
        let real_path = self.as_ref().root.join(child);
        let meta = tokio::fs::metadata(&real_path).await?;
        Ok(LocalFile {
            source: self.clone(),
            child: PathBuf::from(child),
            size: meta.len(),
        })
    }
}

#[derive(Debug)]
pub struct LocalFile {
    source: Source,
    child: PathBuf,
    size: u64,
}

impl LocalFile {
    fn real_path(&self) -> PathBuf {
        self.source.0.root.join(&self.child)
    }
}

impl crate::source::prelude::File for LocalFile {
    type Reader = LocalFileReader;

    fn path(&self) -> &Path {
        &self.child
    }

    fn size(&self) -> u64 {
        self.size
    }

    async fn reader(&self) -> std::io::Result<Self::Reader> {
        tokio::fs::OpenOptions::new()
            .read(true)
            .open(self.real_path())
            .await
    }
}

pub type LocalFileReader = tokio::fs::File;

impl crate::source::prelude::FileReader for LocalFileReader {}

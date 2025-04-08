use std::{path::PathBuf, time::SystemTime};

use futures_lite::StreamExt;

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
        Ok(Source {
            root: self.path.clone(),
        })
    }
}

impl From<Source> for super::AnySource {
    fn from(value: Source) -> Self {
        Self::Local(value)
    }
}

#[derive(Debug)]
pub struct Source {
    root: PathBuf,
}

impl Source {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }
}

impl crate::source::prelude::Source for Source {
    async fn healthcheck(&self) -> std::io::Result<()> {
        if self.root.exists() {
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "root directory not found",
            ))
        }
    }

    async fn list(&self, path: &str) -> std::io::Result<Vec<crate::entry::EntryInfo>> {
        let path = self.root.join(path);
        let mut dir = async_fs::read_dir(&path).await?;
        let mut res = Vec::new();
        while let Ok(Some(entry)) = dir.try_next().await {
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
}

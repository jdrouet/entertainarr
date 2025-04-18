use std::{
    ops::Bound,
    path::{Path, PathBuf},
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    time::SystemTime,
};

use tokio::io::{AsyncSeekExt, ReadBuf};

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

    async fn reader(&self, range: (Bound<u64>, Bound<u64>)) -> std::io::Result<Self::Reader> {
        let mut file = tokio::fs::OpenOptions::new()
            .read(true)
            .open(self.real_path())
            .await?;

        let start = match range.0 {
            Bound::Unbounded => 0,
            Bound::Included(start) => {
                file.seek(std::io::SeekFrom::Start(start)).await?;
                start
            }
            Bound::Excluded(start) => {
                file.seek(std::io::SeekFrom::Start(start + 1)).await?;
                start + 1
            }
        };
        let end = match range.1 {
            Bound::Unbounded => self.size,
            Bound::Included(start) => start,
            Bound::Excluded(start) => start - 1,
        };

        Ok(LocalFileReader {
            file,
            end,
            pos: start,
        })
    }
}

pub struct LocalFileReader {
    file: tokio::fs::File,
    end: u64,
    pos: u64,
}

impl crate::source::prelude::FileReader for LocalFileReader {}

impl tokio::io::AsyncRead for LocalFileReader {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let this = &mut *self;

        // EOF: nothing left to read
        if this.pos >= this.end {
            return Poll::Ready(Ok(()));
        }

        let max_bytes = (this.end - this.pos) as usize;
        let available = buf.remaining().min(max_bytes);

        let unfilled = buf.initialize_unfilled();
        let (to_fill, _) = unfilled.split_at_mut(available);

        let mut temp_buf = ReadBuf::new(to_fill);
        let poll = Pin::new(&mut this.file).poll_read(cx, &mut temp_buf);

        if let Poll::Ready(Ok(())) = &poll {
            let n = temp_buf.filled().len();
            this.pos += n as u64;
            buf.advance(n);
        }

        poll
    }
}

#[cfg(test)]
mod tests {
    // YOU NEED TO DOWNLOAD
    // https://download.blender.org/peach/bigbuckbunny_movies/big_buck_bunny_1080p_surround.avi
    // https://download.blender.org/peach/bigbuckbunny_movies/BigBuckBunny_320x180.mp4

    use std::ops::Bound;
    use std::path::PathBuf;

    use tokio::io::AsyncReadExt;

    use crate::source::prelude::{File, Source};

    async fn read_size(mut reader: impl AsyncReadExt + Unpin) -> usize {
        let mut buf = [0u8; 8192]; // 8KB buffer
        let mut total = 0;

        loop {
            match reader.read(&mut buf).await {
                Ok(0) | Err(_) => break,
                Ok(n) => total += n,
            }
        }

        total
    }

    #[tokio::test]
    async fn should_download_entire_file() -> std::io::Result<()> {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("assets");
        let source = super::Source::new(path);
        let file = source.file("BigBuckBunny_320x180.mp4").await?;
        let reader = file.reader((Bound::Unbounded, Bound::Unbounded)).await?;

        let size = read_size(reader).await;
        assert_eq!(size, 64_657_027);
        Ok(())
    }

    #[tokio::test]
    async fn should_download_only_end() -> std::io::Result<()> {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("assets");
        let source = super::Source::new(path);
        let file = source.file("BigBuckBunny_320x180.mp4").await?;
        let reader = file
            .reader((Bound::Included(4_000_000), Bound::Unbounded))
            .await?;

        let size = read_size(reader).await;
        assert_eq!(size, 60_657_027);
        Ok(())
    }

    #[tokio::test]
    async fn should_download_only_start() -> std::io::Result<()> {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("assets");
        let source = super::Source::new(path);
        let file = source.file("BigBuckBunny_320x180.mp4").await?;
        let reader = file
            .reader((Bound::Unbounded, Bound::Included(4_000_000)))
            .await?;

        let size = read_size(reader).await;
        assert_eq!(size, 4_000_000);
        Ok(())
    }

    #[tokio::test]
    async fn should_download_only_middle() -> std::io::Result<()> {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("assets");
        let source = super::Source::new(path);
        let file = source.file("BigBuckBunny_320x180.mp4").await?;
        let reader = file
            .reader((Bound::Included(2_000_000), Bound::Excluded(4_000_001)))
            .await?;

        let size = read_size(reader).await;
        assert_eq!(size, 2_000_000);
        Ok(())
    }
}

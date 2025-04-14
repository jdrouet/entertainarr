use std::path::Path;

use tokio::io::AsyncRead;

pub trait Source {
    type File: File;

    fn healthcheck(&self) -> impl Future<Output = std::io::Result<()>>;
    fn list(
        &self,
        path: &str,
    ) -> impl Future<Output = std::io::Result<Vec<crate::entry::EntryInfo>>>;
    fn file(&self, path: &str) -> impl Future<Output = std::io::Result<Self::File>>;
}

pub trait File {
    type Reader: FileReader;

    fn path(&self) -> &Path;
    fn content_type(&self) -> mime_guess::Mime {
        mime_guess::from_path(self.path()).first_or_octet_stream()
    }
    fn size(&self) -> u64;

    fn reader(&self) -> impl Future<Output = std::io::Result<Self::Reader>>;
}

pub trait FileReader: AsyncRead {}

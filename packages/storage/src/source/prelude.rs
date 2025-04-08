pub trait Source {
    fn healthcheck(&self) -> impl Future<Output = std::io::Result<()>>;
    fn list(
        &self,
        path: &str,
    ) -> impl Future<Output = std::io::Result<Vec<crate::entry::EntryInfo>>>;
}

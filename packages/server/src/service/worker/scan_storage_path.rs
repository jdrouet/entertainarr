use entertainarr_storage::source::prelude::Source;

#[derive(Debug)]
pub(super) struct ScanStoragePath {
    pub name: String,
    pub path: String,
}

impl ScanStoragePath {
    #[tracing::instrument(name = "scan_storage_path", skip_all, fields(source = %self.name, path = %self.path))]
    pub(super) async fn execute(&self, ctx: &super::Context) -> Result<(), super::Error> {
        tracing::debug!("starting scan");
        let Some(source) = ctx.storage.source(&self.name) else {
            tracing::error!(message = "source unknown");
            return Ok(());
        };
        let entries = source.list(&self.path).await?;
        tracing::debug!("found {} entries", entries.len());
        for entry in entries {
            match entry {
                entertainarr_storage::entry::EntryInfo::File(file) => {
                    let _ = ctx.sender.send(super::Action::analyze_file(
                        self.name.clone(),
                        self.path.clone(),
                        file,
                    ));
                }
                entertainarr_storage::entry::EntryInfo::Directory(directory) => {
                    let path = if self.path.is_empty() {
                        directory.name.clone()
                    } else {
                        format!("{}/{}", self.path, directory.name)
                    };
                    let _ = ctx
                        .sender
                        .send(super::Action::scan_storage_path(self.name.clone(), path));
                }
            }
        }
        Ok(())
    }
}

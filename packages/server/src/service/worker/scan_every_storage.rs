#[derive(Debug)]
pub(super) struct ScanEveryStorage;

impl ScanEveryStorage {
    #[tracing::instrument(name = "scan_every_storage", skip_all)]
    pub(super) async fn execute(&self, ctx: &super::Context) -> Result<(), super::Error> {
        for (name, _) in ctx.storage.sources() {
            let _ = ctx.sender.send(super::Action::scan_storage_path(
                name.to_string(),
                "".to_owned(),
            ));
        }
        Ok(())
    }
}

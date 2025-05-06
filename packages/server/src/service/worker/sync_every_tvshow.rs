use entertainarr_database::model;

#[derive(Debug)]
pub(super) struct SyncEveryTVShow;

impl SyncEveryTVShow {
    #[tracing::instrument(name = "sync_every_tvshow", skip_all)]
    pub(super) async fn execute(&self, ctx: &super::Context) -> Result<(), super::Error> {
        let ids = model::tvshow::list_needing_update(ctx.database.as_ref()).await?;
        tracing::debug!(message = "triggering tvshow sync", count = %ids.len());
        for tvshow_id in ids {
            let _ = ctx.sender.send(super::Action::sync_tvshow(tvshow_id));
        }
        Ok(())
    }
}

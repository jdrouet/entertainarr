use entertainarr_database::model;

#[derive(Debug)]
pub(super) struct SyncEveryTVShow;

impl SyncEveryTVShow {
    #[tracing::instrument(name = "sync_every_tvshow", skip_all)]
    pub(super) async fn execute(&self, ctx: &super::Context) -> Result<(), super::Error> {
        let list = model::tvshow::with_followers(ctx.database.as_ref()).await?;
        for tvshow_id in list {
            let _ = ctx.sender.send(super::Action::sync_tvshow(tvshow_id));
        }
        Ok(())
    }
}

#[derive(Debug)]
pub(super) struct SyncTVShow {
    pub tvshow_id: u64,
}

impl SyncTVShow {
    #[tracing::instrument(name = "sync_tvshow", skip_all, fields(tvshow_id = %self.tvshow_id))]
    pub(super) async fn execute(&self, ctx: &super::Context) -> Result<(), crate::view::Error> {
        let mut tx = ctx.database.as_ref().begin().await?;
        crate::view::tvshow::synchronize_tvshow(&mut tx, &ctx.tmdb, self.tvshow_id).await?;
        tx.commit().await?;
        Ok(())
    }
}

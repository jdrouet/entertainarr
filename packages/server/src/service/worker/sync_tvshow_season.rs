#[derive(Debug)]
pub(super) struct SyncTVShowSeason {
    pub tvshow_id: u64,
    pub season_number: u64,
}

impl SyncTVShowSeason {
    #[tracing::instrument(name = "sync_tvshow_season", skip_all, fields(tvshow_id = %self.tvshow_id, season_number = %self.season_number))]
    pub(super) async fn execute(&self, ctx: &super::Context) -> Result<(), crate::view::Error> {
        let mut tx = ctx.database.as_ref().begin().await?;
        crate::view::tvshow::synchronize_tvshow_season(
            &mut tx,
            &ctx.tmdb,
            self.tvshow_id,
            self.season_number,
        )
        .await?;
        tx.commit().await?;
        Ok(())
    }
}

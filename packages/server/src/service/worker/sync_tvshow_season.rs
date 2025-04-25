use tmdb_api::{prelude::Command, tvshow::season::details::TVShowSeasonDetails};

#[derive(Debug)]
pub(super) struct SyncTVShowSeason {
    pub tvshow_id: u64,
    pub season_number: u64,
}

impl SyncTVShowSeason {
    pub(super) async fn execute(&self, ctx: &super::Context) -> Result<(), super::Error> {
        tracing::debug!("fetching details");
        let season = TVShowSeasonDetails::new(self.tvshow_id, self.season_number)
            .execute(ctx.tmdb.as_ref())
            .await?;

        tracing::debug!("storing in database");
        entertainarr_database::model::tvshow_season::upsert_all(
            ctx.database.as_ref(),
            self.tvshow_id,
            std::iter::once(&season.inner),
        )
        .await?;

        entertainarr_database::model::tvshow_episode::upsert_all(
            ctx.database.as_ref(),
            season.inner.id,
            season.episodes.iter().map(|item| &item.inner),
        )
        .await?;
        tracing::debug!("done");
        Ok(())
    }
}

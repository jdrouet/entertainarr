use tmdb_api::{prelude::Command, tvshow::details::TVShowDetails};

#[derive(Debug)]
pub(super) struct SyncTVShow {
    pub tvshow_id: u64,
}

impl SyncTVShow {
    #[tracing::instrument(name = "sync_tvshow", skip_all, fields(tvshow_id = %self.tvshow_id))]
    pub(super) async fn execute(&self, ctx: &super::Context) -> Result<(), super::Error> {
        tracing::debug!("fetching details");
        let tvshow = TVShowDetails::new(self.tvshow_id)
            .execute(ctx.tmdb.as_ref())
            .await?;

        tracing::debug!("storing in database");
        entertainarr_database::model::tvshow::upsert_all(
            ctx.database.as_ref(),
            std::iter::once(&tvshow.inner),
        )
        .await?;

        tracing::debug!("delegating new tasks");
        for action in tvshow.seasons.into_iter().map(|season| {
            super::Action::sync_tvshow_season(self.tvshow_id, season.inner.season_number)
        }) {
            let _ = ctx.sender.send(action);
        }

        tracing::debug!("done");
        Ok(())
    }
}

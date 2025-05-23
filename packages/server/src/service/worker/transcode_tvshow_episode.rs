#[derive(Debug)]
pub(super) struct TranscodeTVShowEpisode {
    pub episode_id: u64,
}

impl TranscodeTVShowEpisode {
    #[tracing::instrument(name = "transcode_tvshow_episode", skip_all, fields(tvshow_id = %self.episode_id))]
    pub(super) async fn execute(&self, _ctx: &super::Context) -> Result<(), crate::view::Error> {
        tracing::error!("NOT IMPLEMENTED");
        Ok(())
    }
}

use crate::entity::podcast_episode::PodcastEpisode;

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct HomeModel {
    pub podcast_episodes: Vec<PodcastEpisode>,
    pub podcast_episodes_loading: bool,
}

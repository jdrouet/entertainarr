use crate::entity::podcast_episode::PodcastEpisode;

mod execute;
mod update;

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct HomeModel {
    pub podcast_episodes: Vec<PodcastEpisode>,
    pub podcast_episodes_loading: bool,
    pub podcast_episodes_error: bool,
}

impl HomeModel {
    pub fn on_mount(&self) -> crate::ApplicationCommand {
        crate::ApplicationCommand::event(HomeEvent::ListPodcastEpisodesRequest.into())
    }
}

#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    derive_more::From,
    facet::Facet,
    serde::Serialize,
    serde::Deserialize,
)]
#[repr(C)]
pub enum HomeEvent {
    ListPodcastEpisodesRequest,
    ListPodcastEpisodesSuccess(Vec<PodcastEpisode>),
    ListPodcastEpisodesError,
}

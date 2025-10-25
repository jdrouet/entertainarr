mod execute;
mod update;

#[derive(Default)]
pub struct Model {
    pub podcast_episodes: Vec<crate::entity::podcast_episode::PodcastEpisode>,
    pub podcast_episodes_loading: bool,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct View {
    pub podcast_episodes: Vec<crate::entity::podcast_episode::PodcastEpisode>,
    pub podcast_episodes_loading: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, facet::Facet, serde::Serialize, serde::Deserialize)]
#[repr(C)]
pub enum Event {
    Initialize,
    ListPodcastEpisodesRequest,
    ListPodcastEpisodesSuccess(Vec<crate::entity::podcast_episode::PodcastEpisode>),
    ListPodcastEpisodesError(crux_http::HttpError),
}

impl Event {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Initialize => "home.initialize",
            Self::ListPodcastEpisodesRequest => "home.list_podcast_episodes_request",
            Self::ListPodcastEpisodesSuccess(_) => "home.list_podcast_episodes_success",
            Self::ListPodcastEpisodesError(_) => "home.list_podcast_episodes_error",
        }
    }
}

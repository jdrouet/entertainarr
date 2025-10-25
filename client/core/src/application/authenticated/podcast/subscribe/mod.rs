mod execute;
mod update;

#[derive(Clone, Debug, Eq, PartialEq, facet::Facet, serde::Serialize, serde::Deserialize)]
#[repr(C)]
pub enum PodcastSubscribeError {
    InvalidUrl,
    Network,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PodcastSubscribeModel {
    pub loading: bool,
    pub error: Option<PodcastSubscribeError>,
}

#[derive(Clone, Debug, Eq, PartialEq, facet::Facet, serde::Serialize, serde::Deserialize)]
#[repr(C)]
pub struct PodcastSubscribeRequest {
    pub url: String,
}

#[derive(Clone, Debug, Eq, PartialEq, facet::Facet, serde::Serialize, serde::Deserialize)]
#[repr(C)]
pub enum PodcastSubscribeEvent {
    Submit(PodcastSubscribeRequest),
    Success,
    Error(PodcastSubscribeError),
}

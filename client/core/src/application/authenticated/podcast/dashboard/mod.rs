use crate::{
    effect::http::{HttpError, Operation},
    entity::podcast::Podcast,
};

mod execute;
mod update;

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PodcastDashboardModel {
    pub data: Vec<Podcast>,
    pub error: Option<HttpError>,
    pub loading: bool,
}

impl PodcastDashboardModel {
    pub fn on_mount(&self) -> crate::ApplicationCommand {
        crate::ApplicationCommand::event(
            PodcastDashboardEvent::ListPodcastSubscription(Operation::Request(())).into(),
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, facet::Facet, serde::Serialize, serde::Deserialize)]
#[repr(C)]
pub enum PodcastDashboardEvent {
    ListPodcastSubscription(Operation<(), Vec<Podcast>>),
}

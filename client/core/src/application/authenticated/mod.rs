pub mod home;
pub mod podcast;

#[derive(Debug)]
pub enum AuthenticatedModel {
    Home(home::HomeModel),
    PodcastSubscribe(podcast::subscribe::PodcastSubscribeModel),
}

impl AuthenticatedModel {
    pub fn home() -> AuthenticatedModel {
        Self::Home(Default::default())
    }
}

#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    derive_more::From,
    // facet::Facet,
    serde::Serialize,
    serde::Deserialize,
)]
// #[repr(C)]
pub enum AuthenticatedEvent {}

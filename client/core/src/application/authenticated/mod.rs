use crux_core::render::render;

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

    pub fn on_mount(&self) -> crate::ApplicationCommand {
        match self {
            Self::Home(inner) => inner.on_mount(),
            Self::PodcastSubscribe(_) => render(),
        }
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

#![allow(unexpected_cfgs, reason = "typegen is noisy")]

use crux_core::Command;
pub use crux_core::Core;
use crux_core::macros::effect;
use crux_core::render::RenderOperation;
use crux_core::render::render;
use crux_http::protocol::HttpRequest;

use crate::domain::AuthenticatedModel;

pub mod capability;
pub mod domain;
pub mod entity;

pub enum Model {
    Initializing,
    Authentication {
        model: crate::domain::authentication::AuthenticationModel,
        server_url: String,
    },
    Authenticated {
        authentication_token: String,
        model: AuthenticatedModel,
        server_url: String,
    },
}

impl Default for Model {
    fn default() -> Self {
        Self::Initializing
    }
}

impl Model {
    fn server_url(&self) -> Option<&str> {
        match self {
            Self::Initializing => None,
            Self::Authentication { server_url, .. } | Self::Authenticated { server_url, .. } => {
                Some(server_url.as_str())
            }
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ViewModel {
    pub view: View,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum View {
    Authentication(crate::domain::authentication::AuthenticationView),
    Init(crate::domain::init::InitView),
    Home(crate::domain::home::HomeView),
}

impl Default for View {
    fn default() -> Self {
        Self::Init(Default::default())
    }
}

#[effect(typegen)]
#[derive(Debug)]
pub enum Effect {
    Http(HttpRequest),
    Persistence(crate::capability::persistence::Persistence),
    Render(RenderOperation),
}

#[derive(Clone, Debug, derive_more::From, serde::Serialize, serde::Deserialize)]
pub enum Event {
    Authentication(crate::domain::authentication::AuthenticationEvent),
    Home(crate::domain::home::HomeEvent),
    Init(crate::domain::init::InitEvent),
    Noop,
}

impl Event {
    fn name(&self) -> &'static str {
        match self {
            Self::Authentication(inner) => inner.name(),
            Self::Home(inner) => inner.name(),
            Self::Init(inner) => inner.name(),
            Self::Noop => "noop",
        }
    }
}

#[derive(Default)]
pub struct Application;

impl crux_core::App for Application {
    type Capabilities = ();
    type Effect = crate::Effect;
    type Event = crate::Event;
    type Model = crate::Model;
    type ViewModel = crate::ViewModel;

    fn update(
        &self,
        msg: Self::Event,
        model: &mut Self::Model,
        _caps: &(),
    ) -> Command<Self::Effect, Self::Event> {
        tracing::info!(name = msg.name(), "handle event");
        match msg {
            Self::Event::Noop => render(),
            Self::Event::Authentication(event) => self.update_authentication(event, model),
            Self::Event::Home(event) => self.update_home(event, model),
            Self::Event::Init(event) => self.update_init(event, model),
        }
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        match model {
            Model::Initializing => ViewModel {
                view: View::Init(Default::default()),
            },
            Model::Authentication { model, .. } => ViewModel {
                view: View::Authentication(crate::domain::authentication::AuthenticationView {
                    error: model.error.clone(),
                    loading: model.loading,
                }),
            },
            Model::Authenticated { model, .. } => ViewModel {
                view: match model {
                    AuthenticatedModel::Home(inner) => View::Home(crate::domain::home::HomeView {
                        podcast_episodes: inner.podcast_episodes.clone(),
                        podcast_episodes_loading: inner.podcast_episodes_loading,
                    }),
                },
            },
        }
    }
}

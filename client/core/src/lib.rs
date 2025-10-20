#![allow(unexpected_cfgs, reason = "typegen is noisy")]

use crux_core::Command;
pub use crux_core::Core;
use crux_core::macros::effect;
use crux_core::render::RenderOperation;
use crux_core::render::render;
use crux_http::protocol::HttpRequest;

use crate::authentication::api::LoginPayload;

pub mod authentication;
pub mod home;
pub mod init;

// ANCHOR: model

#[derive(Default)]
pub struct Model {
    server_url: Option<String>,
    auth_token: Option<String>,
}

// ANCHOR_END: model

#[derive(Debug, Clone, PartialEq, Eq, facet::Facet, serde::Serialize, serde::Deserialize)]
#[repr(C)]
pub enum HttpResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T> From<crux_http::Result<crux_http::Response<T>>>
    for HttpResult<crux_http::Response<T>, crux_http::HttpError>
{
    fn from(value: crux_http::Result<crux_http::Response<T>>) -> Self {
        match value {
            Ok(response) => HttpResult::Ok(response),
            Err(error) => HttpResult::Err(error),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, facet::Facet, serde::Serialize, serde::Deserialize)]
#[repr(C)]
pub enum Event {
    Authentication(crate::authentication::AuthenticationEvent),
    Init(crate::init::InitEvent),
    Noop,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum View {
    Authentication(crate::authentication::AuthenticationView),
    Init(crate::init::InitView),
    Home(crate::home::HomeView),
}

impl Default for View {
    fn default() -> Self {
        Self::Init(Default::default())
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ViewModel {
    pub view: View,
}

#[effect(typegen)]
#[derive(Debug)]
pub enum Effect {
    Render(RenderOperation),
    Http(HttpRequest),
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
        match msg {
            Self::Event::Noop => render(),
            Self::Event::Authentication(crate::authentication::AuthenticationEvent::Execute {
                email,
                password,
                kind,
            }) => {
                let Some(base_url) = model.server_url.as_deref() else {
                    return render();
                };
                crate::authentication::api::execute(
                    base_url,
                    kind,
                    &LoginPayload { email, password },
                )
            }
            Self::Event::Authentication(crate::authentication::AuthenticationEvent::Callback(
                HttpResult::Ok(mut res),
            )) => {
                let payload = res.take_body().unwrap();
                model.auth_token = Some(payload.token);
                render()
            }
            Self::Event::Authentication(crate::authentication::AuthenticationEvent::Callback(
                HttpResult::Err(_),
            )) => render(),
            Self::Event::Init(event) => {
                model.server_url = Some(event.server_url);
                render()
            }
        }
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        if model.server_url.is_none() {
            return ViewModel {
                view: View::Init(Default::default()),
            };
        }
        if model.auth_token.is_none() {
            return ViewModel {
                view: View::Authentication(Default::default()),
            };
        }
        ViewModel {
            view: View::Home(home::HomeView::default()),
        }
    }
}

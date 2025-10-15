#![allow(unexpected_cfgs, reason = "typegen is noisy")]

use crux_core::Command;
use crux_core::macros::effect;
use crux_core::render::RenderOperation;
use crux_core::render::render;
use crux_http::protocol::HttpRequest;

use crate::authentication::api::LoginPayload;

pub mod authentication;
pub mod init;

// ANCHOR: model

#[derive(Default)]
pub struct Model {
    server_url: Option<String>,
    auth_token: Option<String>,
}

// ANCHOR_END: model

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub enum Event {
    Authentication(crate::authentication::Event),
    Init(crate::init::Event),
}

pub enum View {
    Authentication(crate::authentication::View),
    Init(crate::init::View),
}

impl Default for View {
    fn default() -> Self {
        Self::Init(Default::default())
    }
}

pub struct ViewModel {
    pub view: View,
}

#[effect(typegen)]
#[derive(Debug)]
pub enum Effect {
    #[serde(skip)]
    Render(RenderOperation),
    #[serde(skip)]
    Http(HttpRequest),
}

#[derive(Default)]
pub struct Application;

impl crux_core::App for Application {
    type Model = crate::Model;
    type Event = crate::Event;
    type ViewModel = crate::ViewModel;
    type Capabilities = ();
    type Effect = crate::Effect;

    fn update(
        &self,
        msg: Self::Event,
        model: &mut Self::Model,
        _caps: &(),
    ) -> Command<Self::Effect, Self::Event> {
        match msg {
            Self::Event::Authentication(crate::authentication::Event::Login {
                email,
                password,
            }) => {
                let Some(server_url) = model.server_url.as_ref() else {
                    return render();
                };
                crate::authentication::api::login(server_url, &LoginPayload { email, password })
            }
            Self::Event::Authentication(crate::authentication::Event::Signup {
                email,
                password,
            }) => {
                let Some(server_url) = model.server_url.as_ref() else {
                    return render();
                };
                crate::authentication::api::signup(server_url, &LoginPayload { email, password })
            }
            Self::Event::Authentication(crate::authentication::Event::LoginCallback(Ok(
                mut res,
            ))) => {
                let payload = res.take_body().unwrap();
                model.auth_token = Some(payload.token);
                render()
            }
            Self::Event::Authentication(crate::authentication::Event::LoginCallback(Err(_))) => {
                render()
            }
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
            view: View::Authentication(Default::default()),
        }
    }
}

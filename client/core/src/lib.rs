#![allow(unexpected_cfgs, reason = "typegen is noisy")]

use crux_core::Command;
use crux_core::macros::effect;
use crux_core::render::RenderOperation;
use crux_core::render::render;
use crux_http::HttpError;
use crux_http::command::Http;
use crux_http::protocol::HttpRequest;

pub mod authentication;
pub mod init;

// ANCHOR: model

pub enum AuthenticationModel {
    Anonymous {
        base_url: Option<String>,
        email: Option<String>,
    },
    Authenticated {
        base_url: String,
        token: String,
    },
}

impl Default for AuthenticationModel {
    fn default() -> Self {
        Self::Anonymous {
            base_url: None,
            email: None,
        }
    }
}

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

#[effect(typegen)]
#[derive(Debug)]
pub enum Effect {
    #[serde(skip)]
    Render(RenderOperation),
    #[serde(skip)]
    Http(HttpRequest),
}

#[derive(serde::Serialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Default)]
pub struct Application;

impl crux_core::App for Application {
    type Model = crate::Model;
    type Event = crate::Event;
    type ViewModel = crate::View;
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
                let url = format!("{server_url}/api/auth/login");
                Http::post(url)
                    .body_json(&LoginRequest { email, password })
                    .unwrap()
                    .expect_json()
                    .build()
                    .then_send(|res| {
                        Self::Event::Authentication(crate::authentication::Event::LoginCallback(
                            res,
                        ))
                    })
            }
            Self::Event::Authentication(crate::authentication::Event::Signup {
                email,
                password,
            }) => {
                let Some(server_url) = model.server_url.as_ref() else {
                    return render();
                };
                let url = format!("{server_url}/api/auth/signup");
                Http::post(url)
                    .body_json(&LoginRequest { email, password })
                    .unwrap()
                    .expect_json()
                    .build()
                    .then_send(|res| {
                        Self::Event::Authentication(crate::authentication::Event::LoginCallback(
                            res,
                        ))
                    })
            }
            Self::Event::Authentication(crate::authentication::Event::LoginCallback(Ok(
                mut res,
            ))) => {
                let payload = res.take_body().unwrap();
                model.auth_token = Some(payload.token);
                render()
            }
            Self::Event::Authentication(crate::authentication::Event::LoginCallback(Err(
                HttpError::Http {
                    code,
                    message,
                    body: _,
                },
            ))) => {
                eprintln!("code={code} message={message:?}");
                render()
            }
            Self::Event::Authentication(crate::authentication::Event::LoginCallback(Err(err))) => {
                eprintln!("error: {err:?}");
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
            return View::Init(Default::default());
        }
        if model.auth_token.is_none() {
            return View::Authentication(Default::default());
        }
        View::Authentication(Default::default())
    }
}

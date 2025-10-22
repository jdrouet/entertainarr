use crux_core::{Command, render::render};
use crux_http::HttpError;
use entertainarr_adapter_http::entity::{ApiError, auth::AuthenticationRequestAttributes};

pub mod api;

impl crate::Application {
    pub(crate) fn handle_authentication(
        &self,
        event: AuthenticationEvent,
        model: &mut crate::Model,
    ) -> Command<crate::Effect, crate::Event> {
        match event {
            AuthenticationEvent::Execute {
                email,
                password,
                kind,
            } => {
                let Some(base_url) = model.server_url.as_deref() else {
                    return render();
                };
                model.authentication.error = None;
                crate::authentication::api::execute(
                    base_url,
                    kind,
                    AuthenticationRequestAttributes {
                        email: email.into(),
                        password: password.into(),
                    },
                )
            }
            AuthenticationEvent::Success(token) => {
                model.auth_token = Some(token.clone());
                Command::all([
                    crate::capability::persistence::Persistence::store(
                        "authentication-token",
                        token,
                    ),
                    render(),
                ])
            }
            AuthenticationEvent::Error(err) => {
                match &err {
                    HttpError::Http {
                        code: _,
                        message: _,
                        body: Some(body),
                    } => match serde_json::from_slice::<'_, ApiError>(&body) {
                        Ok(payload) => {
                            model.authentication.error = Some(payload.message.to_string());
                        }
                        Err(_) => {
                            model.authentication.error = Some(err.to_string());
                        }
                    },
                    other => {
                        model.authentication.error = Some(other.to_string());
                    }
                }
                render()
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, facet::Facet, serde::Serialize, serde::Deserialize)]
#[repr(C)]
pub enum AuthenticationKind {
    Login,
    Signup,
}

impl AuthenticationKind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Login => "login",
            Self::Signup => "signup",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, facet::Facet, serde::Serialize, serde::Deserialize)]
#[repr(C)]
pub enum AuthenticationEvent {
    Execute {
        email: String,
        password: String,
        kind: AuthenticationKind,
    },
    Success(String),
    Error(crux_http::HttpError),
}

#[derive(Default)]
pub struct AuthenticationModel {
    pub loading: bool,
    pub error: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AuthenticationView {
    pub error: Option<String>,
}

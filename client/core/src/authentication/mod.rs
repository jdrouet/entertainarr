use crate::HttpResult;

pub mod api;

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
    Callback(HttpResult<crux_http::Response<LoginPayload>, crux_http::HttpError>),
}

#[derive(Clone, Debug, PartialEq, Eq, facet::Facet, serde::Serialize, serde::Deserialize)]
#[repr(C)]
pub struct LoginPayload {
    pub token: String,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AuthenticationView {}

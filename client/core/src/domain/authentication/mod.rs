mod execute;
mod update;

#[derive(Clone, Debug, Eq, PartialEq, facet::Facet, serde::Serialize, serde::Deserialize)]
#[repr(C)]
pub enum AuthenticationError {
    EmailConflict,
    EmailTooShort,
    PasswordTooShort,
    InvalidCredentials,
    Network,
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
    Request(AuthenticationRequest),
    Success(String),
    Error(AuthenticationError),
}

impl AuthenticationEvent {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Request(_) => "authentication.request",
            Self::Success(_) => "authentication.success",
            Self::Error(_) => "authentication.error",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, facet::Facet, serde::Serialize, serde::Deserialize)]
#[repr(C)]
pub struct AuthenticationRequest {
    pub email: String,
    pub password: String,
    pub kind: AuthenticationKind,
}

#[derive(Default)]
pub struct AuthenticationModel {
    pub loading: bool,
    pub error: Option<AuthenticationError>,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AuthenticationView {
    pub loading: bool,
    pub error: Option<AuthenticationError>,
}

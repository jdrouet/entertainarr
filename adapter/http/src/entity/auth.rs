use std::borrow::Cow;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct AuthenticationRequest<'a> {
    pub email: Cow<'a, str>,
    pub password: Cow<'a, str>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct AuthenticationResponse {
    pub token: String,
}

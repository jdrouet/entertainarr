use std::borrow::Cow;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct AuthenticationRequestDocument<'a> {
    #[serde(rename = "type")]
    pub kind: monostate::MustBe!("authentication-requests"),
    pub attributes: AuthenticationRequestAttributes<'a>,
}

impl<'a> AuthenticationRequestDocument<'a> {
    pub fn new(email: impl Into<Cow<'a, str>>, password: impl Into<Cow<'a, str>>) -> Self {
        Self {
            kind: Default::default(),
            attributes: AuthenticationRequestAttributes {
                email: email.into(),
                password: password.into(),
            },
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct AuthenticationRequestAttributes<'a> {
    pub email: Cow<'a, str>,
    pub password: Cow<'a, str>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct AuthenticationTokenDocument {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: monostate::MustBe!("authentication-tokens"),
    pub attributes: AuthenticationTokenAttributes,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct AuthenticationTokenAttributes {}

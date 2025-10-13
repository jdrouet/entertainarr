#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub enum Event {
    Login {
        email: String,
        password: String,
    },
    Signup {
        email: String,
        password: String,
    },
    #[serde(skip)]
    LoginCallback(Result<crux_http::Response<LoginPayload>, crux_http::HttpError>),
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize)]
pub struct LoginPayload {
    pub token: String,
}

#[derive(Debug, Default)]
pub struct View {}

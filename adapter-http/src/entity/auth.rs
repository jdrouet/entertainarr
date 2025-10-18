#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct AuthenticationRequest {
    pub email: String,
    pub password: String,
}

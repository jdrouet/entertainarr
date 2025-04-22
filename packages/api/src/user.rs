#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LoginPayload {
    pub username: String,
}

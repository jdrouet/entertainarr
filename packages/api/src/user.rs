#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LoginPayload {
    pub username: String,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: u64,
    pub name: Box<str>,
}

pub mod auth;
pub mod podcast;
pub mod podcast_episode;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResource<T, I = ()> {
    pub data: T,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub includes: Vec<I>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Relation<T> {
    pub data: T,
}

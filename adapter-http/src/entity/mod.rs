pub mod auth;
pub mod podcast;
pub mod podcast_episode;

fn default_includes<T>() -> Vec<T> {
    Vec::new()
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResource<T, I = ()> {
    pub data: T,
    #[serde(default = "default_includes", skip_serializing_if = "Vec::is_empty")]
    pub includes: Vec<I>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Relation<T> {
    pub data: T,
}

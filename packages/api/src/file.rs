use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct File {
    pub path: PathBuf,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

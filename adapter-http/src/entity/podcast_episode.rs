use std::time::Duration;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PodcastEpisodeDocument {
    pub id: u64,
    #[serde(rename = "type")]
    pub kind: monostate::MustBe!("podcast-episodes"),
    pub attributes: PodcastEpisodeAttributes,
    pub relationship: PodcastEpisodeRelationship,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PodcastEpisodeAttributes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<Duration>,
    pub file_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum PodcastEpisodeField {
    PublishedAt,
}

impl Default for PodcastEpisodeField {
    fn default() -> Self {
        Self::PublishedAt
    }
}

#[derive(Debug)]
pub struct ParsePodcastEpisodeFieldError;

impl std::fmt::Display for ParsePodcastEpisodeFieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("invalid podcast episode field")
    }
}

impl std::str::FromStr for PodcastEpisodeField {
    type Err = ParsePodcastEpisodeFieldError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "published-at" => Ok(Self::PublishedAt),
            _ => Err(ParsePodcastEpisodeFieldError),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum PodcastEpisodeInclude {
    Podcast,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum PodcastEpisodeRelation {
    Podcast(super::podcast::PodcastDocument),
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PodcastEpisodeRelationship {
    pub podcast: super::Relation<super::podcast::PodcastEntity>,
}

use std::time::Duration;

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PodcastEpisodeDocument {
    pub id: u64,
    #[serde(rename = "type")]
    pub kind: monostate::MustBe!("podcast-episodes"),
    pub attributes: PodcastEpisodeAttributes,
    pub relationship: PodcastEpisodeRelationship,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
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

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum PodcastEpisodeField {
    #[default]
    PublishedAt,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum PodcastEpisodeInclude {
    Podcast,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum PodcastEpisodeRelation {
    Podcast(super::podcast::PodcastDocument),
}

impl PodcastEpisodeRelation {
    pub fn into_podcast(self) -> Option<super::podcast::PodcastDocument> {
        match self {
            Self::Podcast(inner) => Some(inner),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PodcastEpisodeRelationship {
    pub podcast: super::Relation<super::podcast::PodcastEntity>,
}

use std::{str::FromStr, time::Duration};

use axum::routing::get;

use crate::{
    adapter::http_server::handler::{Relation, podcast::PodcastEntity},
    domain::podcast::entity::PodcastEpisode,
};

pub mod list;

pub fn create<S>() -> axum::Router<S>
where
    S: crate::adapter::http_server::prelude::ServerState + Clone,
{
    axum::Router::new().route("/podcast-episodes", get(list::handle::<S>))
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PodcastEpisodeDocument {
    pub id: u64,
    #[serde(rename = "type")]
    pub kind: monostate::MustBe!("podcast-episodes"),
    pub attributes: PodcastEpisodeAttributes,
    pub relationship: PodcastEpisodeRelationship,
}

impl From<PodcastEpisode> for PodcastEpisodeDocument {
    fn from(value: PodcastEpisode) -> Self {
        Self {
            id: value.id,
            kind: Default::default(),
            attributes: PodcastEpisodeAttributes {
                guid: value.guid,
                published_at: value.published_at,
                title: value.title,
                description: value.description,
                link: value.link,
                duration: value.duration,
                file_url: value.file_url,
                file_size: value.file_size,
                file_type: value.file_type,
                created_at: value.created_at,
                updated_at: value.updated_at,
            },
            relationship: PodcastEpisodeRelationship {
                podcast: Relation {
                    data: PodcastEntity {
                        id: value.podcast_id,
                        kind: Default::default(),
                    },
                },
            },
        }
    }
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PodcastEpisodeAttributes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Duration>,
    pub file_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PodcastEpisodeField {
    PublishedAt,
}

impl Default for PodcastEpisodeField {
    fn default() -> Self {
        Self::PublishedAt
    }
}

impl From<PodcastEpisodeField> for crate::domain::podcast::prelude::PodcastEpisodeField {
    fn from(value: PodcastEpisodeField) -> Self {
        match value {
            PodcastEpisodeField::PublishedAt => {
                crate::domain::podcast::prelude::PodcastEpisodeField::PublishedAt
            }
        }
    }
}

#[derive(Debug)]
pub struct ParsePodcastEpisodeFieldError;

impl std::fmt::Display for ParsePodcastEpisodeFieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("invalid podcast episode field")
    }
}

impl FromStr for PodcastEpisodeField {
    type Err = ParsePodcastEpisodeFieldError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "published-at" => Ok(Self::PublishedAt),
            _ => Err(ParsePodcastEpisodeFieldError),
        }
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PodcastEpisodeInclude {
    Podcast,
}

#[derive(Debug, serde::Serialize)]
#[serde(untagged)]
pub enum PodcastEpisodeRelation {
    Podcast(super::podcast::PodcastDocument),
}

#[derive(Debug, serde::Serialize)]
pub struct PodcastEpisodeRelationship {
    podcast: Relation<super::podcast::PodcastEntity>,
}

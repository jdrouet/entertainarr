use std::str::FromStr;

use crate::entity::{
    Relation,
    podcast::PodcastEntity,
    podcast_episode::{
        PodcastEpisodeAttributes, PodcastEpisodeDocument, PodcastEpisodeRelationship,
    },
};
use axum::routing::get;

use entertainarr_domain::podcast::entity::PodcastEpisode;

pub mod list;

pub fn create<S>() -> axum::Router<S>
where
    S: crate::server::prelude::ServerState + Clone,
{
    axum::Router::new().route("/podcast-episodes", get(list::handle::<S>))
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

impl From<PodcastEpisodeField> for entertainarr_domain::podcast::prelude::PodcastEpisodeField {
    fn from(value: PodcastEpisodeField) -> Self {
        match value {
            PodcastEpisodeField::PublishedAt => {
                entertainarr_domain::podcast::prelude::PodcastEpisodeField::PublishedAt
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

use std::collections::HashMap;

use entertainarr_adapter_http::entity::ApiResource;
use entertainarr_adapter_http::entity::podcast_episode::{
    PodcastEpisodeDocument, PodcastEpisodeRelation,
};

#[derive(Clone, Debug, Eq, PartialEq, facet::Facet, serde::Serialize, serde::Deserialize)]
pub struct PodcastEpisode {
    pub id: u64,
    pub title: String,
    // TODO fix chrono usage
    // #[serde(default, skip_serializing_if = "Option::is_none")]
    // pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub published_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub podcast_title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub podcast_image_url: Option<String>,
}

impl PodcastEpisode {
    pub fn from_document(
        item: PodcastEpisodeDocument,
        podcasts: &HashMap<u64, entertainarr_adapter_http::entity::podcast::PodcastDocument>,
    ) -> Self {
        let podcast = podcasts.get(&item.relationship.podcast.data.id);
        Self {
            id: item.id,
            title: item.attributes.title,
            published_at: item.attributes.published_at.map(|dt| dt.timestamp()),
            podcast_title: podcast.map(|item| item.attributes.title.clone()),
            podcast_image_url: podcast.and_then(|item| item.attributes.image_url.clone()),
        }
    }

    pub fn from_episode_document_list(
        res: ApiResource<Vec<PodcastEpisodeDocument>, PodcastEpisodeRelation>,
    ) -> Vec<Self> {
        let ApiResource { data, includes } = res;
        let podcasts: HashMap<u64, entertainarr_adapter_http::entity::podcast::PodcastDocument> =
            HashMap::from_iter(
                includes
                    .into_iter()
                    .filter_map(|item| item.into_podcast())
                    .map(|item| (item.id, item)),
            );
        data.into_iter()
            .map(|item| PodcastEpisode::from_document(item, &podcasts))
            .collect()
    }
}

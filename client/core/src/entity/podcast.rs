use entertainarr_adapter_http::entity::{ApiResource, podcast::PodcastDocument};

#[derive(Clone, Debug, Eq, PartialEq, facet::Facet, serde::Serialize, serde::Deserialize)]
pub struct Podcast {
    pub id: u64,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    pub feed_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
}

impl Podcast {
    pub fn from_document(item: PodcastDocument) -> Self {
        Self {
            id: item.id,
            title: item.attributes.title,
            description: item.attributes.description,
            image_url: item.attributes.image_url,
            language: item.attributes.language,
            feed_url: item.attributes.feed_url,
            website: item.attributes.website,
        }
    }

    pub fn from_document_list(res: ApiResource<Vec<PodcastDocument>>) -> Vec<Self> {
        res.data.into_iter().map(Self::from_document).collect()
    }
}

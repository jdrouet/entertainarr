#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PodcastDocument {
    pub id: u64,
    #[serde(rename = "type")]
    pub kind: monostate::MustBe!("podcasts"),
    pub attributes: PodcastAttributes,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PodcastEntity {
    pub id: u64,
    #[serde(rename = "type")]
    pub kind: monostate::MustBe!("podcasts"),
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PodcastAttributes {
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
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PodcastSubscribeDocument {
    #[serde(rename = "type")]
    pub kind: monostate::MustBe!("podcasts"),
    pub attributes: PodcastSubscribeAttributes,
}

impl PodcastSubscribeDocument {
    pub fn new(feed_url: impl Into<String>) -> Self {
        Self {
            kind: Default::default(),
            attributes: PodcastSubscribeAttributes {
                feed_url: feed_url.into(),
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PodcastSubscribeAttributes {
    pub feed_url: String,
}

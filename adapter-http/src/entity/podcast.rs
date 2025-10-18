use std::borrow::Cow;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionRequest<'a> {
    pub feed_url: Cow<'a, str>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PodcastDocument {
    pub id: u64,
    #[serde(rename = "type")]
    pub kind: monostate::MustBe!("podcasts"),
    pub attributes: PodcastAttributes,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PodcastEntity {
    pub id: u64,
    #[serde(rename = "type")]
    pub kind: monostate::MustBe!("podcasts"),
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
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

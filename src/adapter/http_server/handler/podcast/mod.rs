use axum::routing::{delete, post};

pub mod subscribe;
pub mod unsubscribe;
pub mod user_list;

pub fn create<S>() -> axum::Router<S>
where
    S: crate::adapter::http_server::prelude::ServerState + Clone,
{
    axum::Router::new()
        .route(
            "/users/me/podcasts",
            post(subscribe::handle::<S>).get(user_list::handle::<S>),
        )
        .route(
            "/users/me/podcasts/{podcast_id}",
            delete(unsubscribe::handle::<S>),
        )
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PodcastDocument {
    pub id: u64,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<crate::domain::podcast::entity::Podcast> for PodcastDocument {
    fn from(value: crate::domain::podcast::entity::Podcast) -> Self {
        Self {
            id: value.id,
            title: value.title,
            description: value.description,
            image_url: value.image_url,
            language: value.language,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

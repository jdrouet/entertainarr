use crate::entity::podcast::{PodcastAttributes, PodcastDocument};
use axum::routing::{delete, post};

pub mod subscribe;
pub mod unsubscribe;
pub mod user_list;

pub fn create<S>() -> axum::Router<S>
where
    S: crate::server::prelude::ServerState + Clone,
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

impl From<entertainarr_domain::podcast::entity::Podcast> for PodcastDocument {
    fn from(value: entertainarr_domain::podcast::entity::Podcast) -> Self {
        Self {
            id: value.id,
            kind: Default::default(),
            attributes: PodcastAttributes {
                title: value.title,
                description: value.description,
                image_url: value.image_url,
                language: value.language,
                feed_url: value.feed_url,
                website: value.website,
                created_at: value.created_at,
                updated_at: value.updated_at,
            },
        }
    }
}

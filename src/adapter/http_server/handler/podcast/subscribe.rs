use axum::Json;
use axum::extract::State;

use crate::adapter::http_server::extractor::user::CurrentUser;
use crate::adapter::http_server::handler::ApiError;
use crate::domain::podcast::prelude::PodcastService;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscribePayload {
    feed_url: String,
}

pub async fn handle<S>(
    State(state): State<S>,
    CurrentUser(user_id): CurrentUser,
    Json(payload): Json<SubscribePayload>,
) -> Result<axum::http::StatusCode, ApiError>
where
    S: crate::adapter::http_server::prelude::ServerState,
{
    state
        .podcast_service()
        .subscribe(user_id, &payload.feed_url)
        .await
        .map(|_| axum::http::StatusCode::CREATED)
        .map_err(|err| {
            tracing::error!(error = ?err, "unable to insert subscription");
            ApiError::internal()
        })
}

#[cfg(test)]
mod tests {
    use axum::{Json, extract::State, http::StatusCode};
    use chrono::Utc;

    use crate::{
        adapter::http_server::{extractor::user::CurrentUser, prelude::tests::MockServerState},
        domain::podcast::{entity::Podcast, prelude::MockPodcastService},
    };

    #[tokio::test]
    async fn should_succeed() {
        let mut podcast_service = MockPodcastService::new();
        podcast_service
            .expect_subscribe()
            .return_once(|user_id, feed_url| {
                assert_eq!(user_id, 1);
                let feed_url = feed_url.to_string();
                Box::pin(async move {
                    Ok(Podcast {
                        id: 1,
                        feed_url,
                        title: "foo".into(),
                        description: None,
                        image_url: None,
                        language: None,
                        website: None,
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                    })
                })
            });
        let state = MockServerState::builder().podcast(podcast_service).build();
        let payload = super::SubscribePayload {
            feed_url: "http://example.org/feed.rss".into(),
        };
        let res = super::handle(State(state), CurrentUser(1), Json(payload))
            .await
            .unwrap();
        assert_eq!(res, StatusCode::CREATED);
    }
}

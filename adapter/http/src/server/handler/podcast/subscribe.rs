use axum::Json;
use axum::extract::State;

use crate::entity::podcast::SubscriptionRequest;
use crate::server::extractor::user::CurrentUser;
use crate::server::handler::ApiError;
use entertainarr_domain::podcast::prelude::PodcastService;

pub async fn handle<S>(
    State(state): State<S>,
    CurrentUser(user_id): CurrentUser,
    Json(payload): Json<SubscriptionRequest<'static>>,
) -> Result<axum::http::StatusCode, ApiError>
where
    S: crate::server::prelude::ServerState,
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
    use crate::entity::podcast::SubscriptionRequest;
    use crate::server::{extractor::user::CurrentUser, prelude::tests::MockServerState};

    use axum::{Json, extract::State, http::StatusCode};
    use chrono::Utc;
    use entertainarr_domain::podcast::{entity::Podcast, prelude::MockPodcastService};

    #[tokio::test]
    async fn should_fail_if_service_fails() {
        let mut podcast_service = MockPodcastService::new();
        podcast_service
            .expect_subscribe()
            .return_once(|user_id, _feed_url| {
                assert_eq!(user_id, 1);
                Box::pin(async move { Err(anyhow::anyhow!("oops")) })
            });
        let state = MockServerState::builder().podcast(podcast_service).build();
        let payload = SubscriptionRequest {
            feed_url: "http://example.org/feed.rss".into(),
        };
        let err = super::handle(State(state), CurrentUser(1), Json(payload))
            .await
            .unwrap_err();
        assert_eq!(err.status_code, StatusCode::INTERNAL_SERVER_ERROR);
    }

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
        let payload = SubscriptionRequest {
            feed_url: "http://example.org/feed.rss".into(),
        };
        let res = super::handle(State(state), CurrentUser(1), Json(payload))
            .await
            .unwrap();
        assert_eq!(res, StatusCode::CREATED);
    }
}

use axum::Json;
use axum::extract::State;

use crate::adapter::http_server::extractor::user::CurrentUser;
use crate::adapter::http_server::handler::{ApiError, ApiResource};
use crate::domain::podcast::prelude::PodcastService;

pub async fn handle<S>(
    State(state): State<S>,
    CurrentUser(user_id): CurrentUser,
) -> Result<Json<ApiResource<Vec<super::PodcastDocument>>>, ApiError>
where
    S: crate::adapter::http_server::prelude::ServerState,
{
    let list = state
        .podcast_service()
        .subscriptions(user_id)
        .await
        .map_err(|err| {
            tracing::error!(error = ?err, "unable to list user podcasts");
            ApiError::internal()
        })?;
    Ok(Json(ApiResource {
        data: list
            .into_iter()
            .map(super::PodcastDocument::from)
            .collect::<Vec<_>>(),
    }))
}

#[cfg(test)]
mod integration {

    use tower::ServiceExt;

    use crate::{
        adapter::http_server::prelude::tests::MockServerState,
        domain::{
            auth::{entity::Profile, prelude::MockAuthenticationService},
            podcast::prelude::MockPodcastService,
        },
    };

    #[tokio::test]
    async fn should_fail_if_anonymous() {
        let router = crate::adapter::http_server::handler::create();
        let state = MockServerState::builder().build();
        let res = router
            .with_state(state)
            .oneshot(
                axum::http::Request::builder()
                    .uri("/api/users/me/podcasts")
                    .method(axum::http::Method::GET)
                    .header("Content-Type", "application/json")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), axum::http::StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn should_fail_if_token_malformed() {
        let router = crate::adapter::http_server::handler::create();
        let state = MockServerState::builder().build();
        let res = router
            .with_state(state)
            .oneshot(
                axum::http::Request::builder()
                    .uri("/api/users/me/podcasts")
                    .method(axum::http::Method::GET)
                    .header("Content-Type", "application/json")
                    .header("Authorization", "nope")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), axum::http::StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn should_answer_if_autheticated() {
        let router = crate::adapter::http_server::handler::create();
        let mut auth_service = MockAuthenticationService::new();
        auth_service
            .expect_verify()
            .returning(|_| Box::pin(async { Ok(Profile { id: 1 }) }));
        let mut podcast_service = MockPodcastService::new();
        podcast_service
            .expect_subscriptions()
            .returning(|_| Box::pin(async { Ok(Vec::default()) }));
        let state = MockServerState::builder()
            .authentication(auth_service)
            .podcast(podcast_service)
            .build();
        let res = router
            .with_state(state)
            .oneshot(
                axum::http::Request::builder()
                    .uri("/api/users/me/podcasts")
                    .method(axum::http::Method::GET)
                    .header("Authorization", "Bearer fake")
                    .header("Content-Type", "application/json")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), axum::http::StatusCode::OK);
    }
}

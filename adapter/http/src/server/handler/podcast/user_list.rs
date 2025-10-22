use axum::Json;
use axum::extract::State;
use entertainarr_domain::podcast::prelude::PodcastService;

use crate::entity::podcast::PodcastDocument;
use crate::entity::{ApiError, ApiResource};
use crate::server::extractor::user::CurrentUser;

pub async fn handle<S>(
    State(state): State<S>,
    CurrentUser(user_id): CurrentUser,
) -> Result<Json<ApiResource<Vec<PodcastDocument>>>, ApiError>
where
    S: crate::server::prelude::ServerState,
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
            .map(PodcastDocument::from)
            .collect::<Vec<_>>(),
        includes: Vec::new(),
    }))
}

#[cfg(test)]
mod integration {

    use chrono::Utc;
    use tower::ServiceExt;

    use crate::server::prelude::tests::MockServerState;
    use entertainarr_domain::{
        auth::{entity::Profile, prelude::MockAuthenticationService},
        podcast::{entity::Podcast, prelude::MockPodcastService},
    };

    #[tokio::test]
    async fn should_fail_if_anonymous() {
        let router = crate::server::handler::create();
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
        let router = crate::server::handler::create();
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
    async fn should_fail_if_token_expired() {
        let router = crate::server::handler::create();
        let mut auth_service = MockAuthenticationService::new();
        auth_service.expect_verify().returning(|token| {
            assert_eq!(token, "token");
            Box::pin(
                async move { Err(entertainarr_domain::auth::prelude::VerifyError::ExpiredToken) },
            )
        });
        let state = MockServerState::builder()
            .authentication(auth_service)
            .build();
        let res = router
            .with_state(state)
            .oneshot(
                axum::http::Request::builder()
                    .uri("/api/users/me/podcasts")
                    .method(axum::http::Method::GET)
                    .header("Content-Type", "application/json")
                    .header("Authorization", "Bearer token")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), axum::http::StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn should_fail_if_token_invalid() {
        let router = crate::server::handler::create();
        let mut auth_service = MockAuthenticationService::new();
        auth_service.expect_verify().returning(|token| {
            assert_eq!(token, "token");
            Box::pin(
                async move { Err(entertainarr_domain::auth::prelude::VerifyError::InvalidToken) },
            )
        });
        let state = MockServerState::builder()
            .authentication(auth_service)
            .build();
        let res = router
            .with_state(state)
            .oneshot(
                axum::http::Request::builder()
                    .uri("/api/users/me/podcasts")
                    .method(axum::http::Method::GET)
                    .header("Content-Type", "application/json")
                    .header("Authorization", "Bearer token")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), axum::http::StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn should_fail_if_token_failed_decoding() {
        let router = crate::server::handler::create();
        let mut auth_service = MockAuthenticationService::new();
        auth_service.expect_verify().returning(|token| {
            assert_eq!(token, "token");
            Box::pin(async move {
                Err(entertainarr_domain::auth::prelude::VerifyError::Internal(
                    anyhow::anyhow!("oops"),
                ))
            })
        });
        let state = MockServerState::builder()
            .authentication(auth_service)
            .build();
        let res = router
            .with_state(state)
            .oneshot(
                axum::http::Request::builder()
                    .uri("/api/users/me/podcasts")
                    .method(axum::http::Method::GET)
                    .header("Content-Type", "application/json")
                    .header("Authorization", "Bearer token")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn should_answer_if_autheticated() {
        let router = crate::server::handler::create();
        let mut auth_service = MockAuthenticationService::new();
        auth_service
            .expect_verify()
            .returning(|_| Box::pin(async { Ok(Profile { id: 1 }) }));
        let mut podcast_service = MockPodcastService::new();
        podcast_service.expect_subscriptions().returning(|_| {
            Box::pin(async {
                Ok(vec![Podcast {
                    id: 1,
                    title: "title".into(),
                    feed_url: "feed".into(),
                    image_url: None,
                    language: None,
                    website: None,
                    description: None,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                }])
            })
        });
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

    #[tokio::test]
    async fn should_fail_if_service_fails() {
        let router = crate::server::handler::create();
        let mut auth_service = MockAuthenticationService::new();
        auth_service
            .expect_verify()
            .returning(|_| Box::pin(async { Ok(Profile { id: 1 }) }));
        let mut podcast_service = MockPodcastService::new();
        podcast_service
            .expect_subscriptions()
            .returning(|_| Box::pin(async { Err(anyhow::anyhow!("oops")) }));
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
        assert_eq!(res.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }
}

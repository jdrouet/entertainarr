use axum::extract::{Path, State};

use crate::adapter::http_server::extractor::user::CurrentUser;
use crate::adapter::http_server::handler::ApiError;
use crate::domain::podcast::prelude::PodcastService;

pub async fn handle<S>(
    State(state): State<S>,
    CurrentUser(user_id): CurrentUser,
    Path(podcast_id): Path<u64>,
) -> Result<axum::http::StatusCode, ApiError>
where
    S: crate::adapter::http_server::prelude::ServerState,
{
    state
        .podcast_service()
        .unsubscribe(user_id, podcast_id)
        .await
        .map(|_| axum::http::StatusCode::NO_CONTENT)
        .map_err(|err| {
            tracing::error!(error = ?err, "unable to delete subscription");
            ApiError::internal()
        })
}

#[cfg(test)]
mod tests {
    use axum::{
        extract::{Path, State},
        http::StatusCode,
    };

    use crate::{
        adapter::http_server::{extractor::user::CurrentUser, prelude::tests::MockServerState},
        domain::podcast::prelude::MockPodcastService,
    };

    #[tokio::test]
    async fn should_fail_if_service_fails() {
        let mut podcast_service = MockPodcastService::new();
        podcast_service
            .expect_unsubscribe()
            .return_once(|user_id, podcast_id| {
                assert_eq!(user_id, 1);
                assert_eq!(podcast_id, 2);
                Box::pin(async move { Err(anyhow::anyhow!("oops")) })
            });
        let state = MockServerState::builder().podcast(podcast_service).build();
        let err = super::handle(State(state), CurrentUser(1), Path(2))
            .await
            .unwrap_err();
        assert_eq!(err.status_code, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn should_succeed() {
        let mut podcast_service = MockPodcastService::new();
        podcast_service
            .expect_unsubscribe()
            .return_once(|user_id, podcast_id| {
                assert_eq!(user_id, 1);
                assert_eq!(podcast_id, 2);
                Box::pin(async { Ok(()) })
            });
        let state = MockServerState::builder().podcast(podcast_service).build();
        let res = super::handle(State(state), CurrentUser(1), Path(2))
            .await
            .unwrap();
        assert_eq!(res, StatusCode::NO_CONTENT);
    }
}

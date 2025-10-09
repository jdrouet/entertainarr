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
        .map(|_| axum::http::StatusCode::OK)
        .map_err(|err| {
            tracing::error!(error = ?err, "unable to delete subscription");
            ApiError::internal()
        })
}

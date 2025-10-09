use axum::Json;
use axum::extract::State;

use crate::adapter::http_server::extractor::user::CurrentUser;
use crate::adapter::http_server::handler::{ApiError, ApiResource};
use crate::domain::podcast::prelude::PodcastService;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct SubscribePayload {
    feed_url: String,
}

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

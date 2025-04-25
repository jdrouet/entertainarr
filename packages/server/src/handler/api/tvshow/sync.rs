use axum::Extension;
use axum::extract::Path;
use axum::http::StatusCode;

use crate::handler::api::error::ApiError;
use crate::service::worker::{Action, Worker};

pub(super) async fn single(
    Extension(worker): Extension<Worker>,
    Path(tvshow_id): Path<u64>,
) -> Result<StatusCode, ApiError> {
    worker.push(Action::sync_tvshow(tvshow_id)).await;
    Ok(StatusCode::CREATED)
}

pub(super) async fn all(Extension(worker): Extension<Worker>) -> Result<StatusCode, ApiError> {
    worker.push(Action::sync_every_tvshow()).await;
    Ok(StatusCode::CREATED)
}

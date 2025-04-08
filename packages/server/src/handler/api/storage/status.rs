use axum::{Extension, http::StatusCode};

use crate::handler::api::error::ApiError;
use crate::service::storage::Storage;

pub(super) async fn handle(Extension(storage): Extension<Storage>) -> Result<StatusCode, ApiError> {
    storage
        .healthcheck()
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(ApiError::from)
}

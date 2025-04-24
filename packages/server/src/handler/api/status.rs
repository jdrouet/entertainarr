use axum::{Extension, http::StatusCode};
use entertainarr_database::Database;

use super::error::ApiError;

pub(super) async fn handle(Extension(db): Extension<Database>) -> Result<StatusCode, ApiError> {
    db.ping()
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(ApiError::from)
}

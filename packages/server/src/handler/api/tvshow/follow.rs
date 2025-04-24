use axum::Extension;
use axum::extract::Path;
use axum::http::StatusCode;

use crate::handler::api::authentication::Authentication;
use crate::handler::api::error::ApiError;
use entertainarr_database::Database;

pub(super) async fn create(
    Extension(db): Extension<Database>,
    Authentication(user_id): Authentication,
    Path(tvshow_id): Path<u64>,
) -> Result<StatusCode, ApiError> {
    entertainarr_database::model::tvshow::follow(db.as_ref(), user_id, tvshow_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub(super) async fn delete(
    Extension(db): Extension<Database>,
    Authentication(user_id): Authentication,
    Path(tvshow_id): Path<u64>,
) -> Result<StatusCode, ApiError> {
    entertainarr_database::model::tvshow::unfollow(db.as_ref(), user_id, tvshow_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

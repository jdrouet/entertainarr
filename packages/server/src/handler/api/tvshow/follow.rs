use axum::http::StatusCode;
use axum::{Extension, Json};

use crate::handler::api::authentication::Authentication;
use crate::handler::api::error::ApiError;
use crate::service::database::Database;

#[derive(Debug, serde::Deserialize)]
pub struct Payload {
    tvshow_id: u64,
}

pub(super) async fn create(
    Extension(db): Extension<Database>,
    Authentication(user_id): Authentication,
    Json(payload): Json<Payload>,
) -> Result<StatusCode, ApiError> {
    crate::model::tvshow::follow(db.as_ref(), user_id, payload.tvshow_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub(super) async fn delete(
    Extension(db): Extension<Database>,
    Authentication(user_id): Authentication,
    Json(payload): Json<Payload>,
) -> Result<StatusCode, ApiError> {
    crate::model::tvshow::unfollow(db.as_ref(), user_id, payload.tvshow_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

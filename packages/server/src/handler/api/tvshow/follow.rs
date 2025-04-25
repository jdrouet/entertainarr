use axum::extract::Path;
use axum::{Extension, Json};
use entertainarr_api::tvshow::TVShow;
use entertainarr_database::Database;
use entertainarr_database::model::tvshow;

use crate::handler::api::authentication::Authentication;
use crate::handler::api::error::ApiError;
use crate::service::worker::{Action, Worker};

use super::tvshow_to_view;

pub(super) async fn create(
    Extension(db): Extension<Database>,
    Extension(worker): Extension<Worker>,
    Authentication(user_id): Authentication,
    Path(tvshow_id): Path<u64>,
) -> Result<Json<TVShow>, ApiError> {
    let mut tx = db.as_ref().begin().await?;
    tvshow::follow(&mut *tx, user_id, tvshow_id).await?;
    let item = tvshow::find_by_id(&mut *tx, user_id, tvshow_id).await?;
    let item = item.ok_or_else(|| ApiError::not_found("tvshow not found"))?;
    tx.commit().await?;
    worker.push(Action::sync_tvshow(tvshow_id)).await;
    Ok(Json(tvshow_to_view(item)))
}

pub(super) async fn delete(
    Extension(db): Extension<Database>,
    Authentication(user_id): Authentication,
    Path(tvshow_id): Path<u64>,
) -> Result<Json<TVShow>, ApiError> {
    let mut tx = db.as_ref().begin().await?;
    tvshow::unfollow(&mut *tx, user_id, tvshow_id).await?;
    let item = tvshow::find_by_id(&mut *tx, user_id, tvshow_id).await?;
    let item = item.ok_or_else(|| ApiError::not_found("tvshow not found"))?;
    tx.commit().await?;
    Ok(Json(tvshow_to_view(item)))
}

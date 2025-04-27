use axum::extract::Path;
use axum::{Extension, Json};
use entertainarr_api::tvshow::TVShow;

use crate::handler::api::authentication::Authentication;
use crate::handler::api::error::ApiError;
use entertainarr_database::{Database, model};

async fn handle(
    db: Database,
    user_id: u64,
    tvshow_id: u64,
    watched: bool,
) -> Result<TVShow, ApiError> {
    let mut tx = db.as_ref().begin().await?;
    if watched {
        model::tvshow::watched(db.as_ref(), user_id, tvshow_id).await?;
    } else {
        model::tvshow::unwatched(db.as_ref(), user_id, tvshow_id).await?;
    }
    let item = model::tvshow::find_by_id(&mut *tx, user_id, tvshow_id).await?;
    let item = item.ok_or_else(|| ApiError::not_found("tvshow not found"))?;
    tx.commit().await?;
    Ok(super::tvshow_to_view(item))
}

pub async fn create(
    Extension(db): Extension<Database>,
    Path(tvshow_id): Path<u64>,
    Authentication(user_id): Authentication,
) -> Result<Json<TVShow>, ApiError> {
    handle(db, user_id, tvshow_id, true).await.map(Json)
}

pub async fn delete(
    Extension(db): Extension<Database>,
    Path(tvshow_id): Path<u64>,
    Authentication(user_id): Authentication,
) -> Result<Json<TVShow>, ApiError> {
    handle(db, user_id, tvshow_id, false).await.map(Json)
}

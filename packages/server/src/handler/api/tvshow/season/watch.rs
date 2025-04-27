use axum::extract::Path;
use axum::{Extension, Json};
use entertainarr_api::tvshow_season::TVShowSeason;

use crate::handler::api::authentication::Authentication;
use crate::handler::api::error::ApiError;
use entertainarr_database::{Database, model};

async fn handle(
    db: Database,
    user_id: u64,
    tvshow_id: u64,
    season_number: u64,
    watched: bool,
) -> Result<TVShowSeason, ApiError> {
    let mut tx = db.as_ref().begin().await?;
    if watched {
        model::tvshow_season::watched(db.as_ref(), user_id, tvshow_id, season_number).await?;
    } else {
        model::tvshow_season::unwatched(db.as_ref(), user_id, tvshow_id, season_number).await?;
    }
    let item =
        model::tvshow_season::find_by_number(&mut *tx, user_id, tvshow_id, season_number).await?;
    let item = item.ok_or_else(|| ApiError::not_found("tvshow season not found"))?;
    tx.commit().await?;
    Ok(super::season_to_view(item))
}

pub async fn create(
    Extension(db): Extension<Database>,
    Path((tvshow_id, season_number)): Path<(u64, u64)>,
    Authentication(user_id): Authentication,
) -> Result<Json<TVShowSeason>, ApiError> {
    handle(db, user_id, tvshow_id, season_number, true)
        .await
        .map(Json)
}

pub async fn delete(
    Extension(db): Extension<Database>,
    Path((tvshow_id, season_number)): Path<(u64, u64)>,
    Authentication(user_id): Authentication,
) -> Result<Json<TVShowSeason>, ApiError> {
    handle(db, user_id, tvshow_id, season_number, false)
        .await
        .map(Json)
}

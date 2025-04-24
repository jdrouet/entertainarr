use axum::Extension;
use axum::extract::Path;
use axum::http::StatusCode;

use crate::handler::api::authentication::Authentication;
use crate::handler::api::error::ApiError;
use entertainarr_database::Database;

pub async fn create(
    Extension(db): Extension<Database>,
    Path((tvshow_id, season_number, episode_number)): Path<(u64, u64, u64)>,
    Authentication(user_id): Authentication,
) -> Result<StatusCode, ApiError> {
    entertainarr_database::model::tvshow_episode::watched(
        db.as_ref(),
        user_id,
        tvshow_id,
        season_number,
        episode_number,
        0,
        true,
    )
    .await?;
    Ok(StatusCode::CREATED)
}

pub async fn delete(
    Extension(db): Extension<Database>,
    Path((tvshow_id, season_number, episode_number)): Path<(u64, u64, u64)>,
    Authentication(user_id): Authentication,
) -> Result<StatusCode, ApiError> {
    entertainarr_database::model::tvshow_episode::unwatched(
        db.as_ref(),
        user_id,
        tvshow_id,
        season_number,
        episode_number,
    )
    .await?;
    Ok(StatusCode::CREATED)
}

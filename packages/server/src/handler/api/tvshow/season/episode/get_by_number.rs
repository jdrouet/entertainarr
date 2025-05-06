use axum::extract::Path;
use axum::{Extension, Json};
use entertainarr_api::tvshow_episode::TVShowEpisode;
use entertainarr_database::Database;

use crate::handler::api::authentication::Authentication;
use crate::handler::api::error::ApiError;

use super::episode_to_view;

pub(super) async fn handle(
    Extension(db): Extension<Database>,
    Authentication(user_id): Authentication,
    Path((tvshow_id, season_number, episode_number)): Path<(u64, u64, u64)>,
) -> Result<Json<TVShowEpisode>, ApiError> {
    let found = entertainarr_database::model::tvshow_episode::find_by_number(
        db.as_ref(),
        user_id,
        tvshow_id,
        season_number,
        episode_number,
    )
    .await?;
    let found = found.ok_or_else(|| ApiError::not_found("episode not found"))?;
    Ok(Json(episode_to_view(found)))
}

use axum::{Extension, Json};
use entertainarr_api::tvshow_episode::TVShowEpisodeSmall;
use entertainarr_database::{Database, model};

use crate::handler::api::{authentication::Authentication, error::ApiError};

pub(super) async fn handle(
    Extension(db): Extension<Database>,
    Authentication(user_id): Authentication,
) -> Result<Json<Vec<TVShowEpisodeSmall>>, ApiError> {
    let list = model::tvshow_episode::watchlist(db.as_ref(), user_id).await?;
    let list = list
        .into_iter()
        .map(|item| TVShowEpisodeSmall {
            tvshow_id: item.tvshow_id,
            tvshow_name: item.tvshow_name,
            season_number: item.season_number,
            episode_number: item.episode_number,
            image_path: item.image_path,
            air_date: item.air_date,
        })
        .collect();
    Ok(Json(list))
}

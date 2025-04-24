use axum::extract::Path;
use axum::{Extension, Json};
use entertainarr_api::tvshow_episode::TVShowEpisode;

use crate::handler::api::authentication::Authentication;
use crate::handler::api::error::ApiError;
use crate::service::database::Database;

pub async fn handle(
    Extension(db): Extension<Database>,
    Path((tvshow_id, season_number)): Path<(u64, u64)>,
    Authentication(_): Authentication,
) -> Result<Json<Vec<TVShowEpisode>>, ApiError> {
    let list = crate::model::tvshow_episode::list(db.as_ref(), tvshow_id, season_number).await?;
    Ok(Json(list.into_iter().map(From::from).collect()))
}

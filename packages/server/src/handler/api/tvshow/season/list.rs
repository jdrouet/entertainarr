use axum::extract::Path;
use axum::{Extension, Json};
use entertainarr_api::tvshow_season::TVShowSeason;

use crate::handler::api::authentication::Authentication;
use crate::handler::api::error::ApiError;
use crate::service::tmdb::Tmdb;
use entertainarr_database::Database;

use super::season_to_view;

pub async fn handle(
    Extension(db): Extension<Database>,
    Extension(tmdb): Extension<Tmdb>,
    Path(tvshow_id): Path<u64>,
    Authentication(user_id): Authentication,
) -> Result<Json<Vec<TVShowSeason>>, ApiError> {
    let list =
        entertainarr_database::model::tvshow_season::list(db.as_ref(), user_id, tvshow_id).await?;
    if !list.is_empty() {
        return Ok(Json(list.into_iter().map(season_to_view).collect()));
    }
    let details = tmdb
        .as_ref()
        .get_tvshow_details(tvshow_id, &Default::default())
        .await?;
    if !details.seasons.is_empty() {
        entertainarr_database::model::tvshow_season::upsert_all(
            db.as_ref(),
            tvshow_id,
            details.seasons.iter().map(|season| &season.inner),
        )
        .await?;
    }

    let list =
        entertainarr_database::model::tvshow_season::list(db.as_ref(), user_id, tvshow_id).await?;
    Ok(Json(list.into_iter().map(super::season_to_view).collect()))
}

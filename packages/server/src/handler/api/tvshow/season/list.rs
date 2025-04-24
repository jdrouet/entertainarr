use axum::extract::Path;
use axum::{Extension, Json};
use entertainarr_api::tvshow_season::TVShowSeason;
use tmdb_api::prelude::Command;
use tmdb_api::tvshow::details::TVShowDetails;

use crate::handler::api::authentication::Authentication;
use crate::handler::api::error::ApiError;
use crate::service::database::Database;
use crate::service::tmdb::Tmdb;

pub async fn handle(
    Extension(db): Extension<Database>,
    Extension(tmdb): Extension<Tmdb>,
    Path(tvshow_id): Path<u64>,
    Authentication(_): Authentication,
) -> Result<Json<Vec<TVShowSeason>>, ApiError> {
    let list = crate::model::tvshow_season::list(db.as_ref(), tvshow_id).await?;
    if !list.is_empty() {
        return Ok(Json(list.into_iter().map(From::from).collect()));
    }
    let details = TVShowDetails::new(tvshow_id).execute(tmdb.as_ref()).await?;
    crate::model::tvshow_season::upsert_all(
        db.as_ref(),
        tvshow_id,
        details.seasons.iter().map(|season| &season.inner),
    )
    .await?;

    let list = details
        .seasons
        .into_iter()
        .map(|season| TVShowSeason::from(season.inner))
        .collect();
    Ok(Json(list))
}

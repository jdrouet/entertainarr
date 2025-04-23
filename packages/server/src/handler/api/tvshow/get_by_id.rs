use axum::extract::Path;
use axum::{Extension, Json};
use entertainarr_api::tvshow::TVShow;
use tmdb_api::prelude::Command;
use tmdb_api::tvshow::details::TVShowDetails;

use crate::handler::api::error::ApiError;
use crate::service::database::Database;
use crate::service::tmdb::Tmdb;

pub(super) async fn handle(
    Extension(db): Extension<Database>,
    Extension(tmdb): Extension<Tmdb>,
    Path(tvshow_id): Path<u64>,
) -> Result<Json<TVShow>, ApiError> {
    let mut tx = db.as_ref().begin().await?;
    if let Some(found) = crate::model::tvshow::find_by_id(&mut *tx, tvshow_id).await? {
        return Ok(Json(found.into()));
    }
    let res = TVShowDetails::new(tvshow_id).execute(tmdb.as_ref()).await?;
    crate::model::tvshow::upsert_all(&mut *tx, std::iter::once(&res.inner)).await?;
    tx.commit().await?;
    Ok(Json(res.inner.into()))
}

use axum::extract::Query;
use axum::{Extension, Json};
use tmdb_api::prelude::Command;
use tmdb_api::tvshow::TVShowBase;
use tmdb_api::tvshow::search::TVShowSearch;

use crate::handler::api::error::ApiError;
use crate::service::tmdb::Tmdb;
use entertainarr_database::Database;

#[derive(Debug, serde::Deserialize)]
pub struct SearchQuery {
    #[serde(alias = "q", default)]
    query: String,
    #[serde(default)]
    page: u32,
}

pub(super) async fn handle(
    Extension(db): Extension<Database>,
    Extension(tmdb): Extension<Tmdb>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<Vec<TVShowBase>>, ApiError> {
    let tvshows = TVShowSearch::new(query.query)
        .with_page(Some(query.page + 1))
        .execute(tmdb.as_ref())
        .await?;
    let list: Vec<TVShowBase> = tvshows.results.into_iter().map(|item| item.inner).collect();
    if list.is_empty() {
        return Ok(Json(Vec::new()));
    }
    entertainarr_database::model::tvshow::upsert_all(db.as_ref(), list.iter()).await?;
    Ok(Json(list))
}

use axum::extract::Query;
use axum::{Extension, Json};
use tmdb_api::common::PaginatedResult;
use tmdb_api::prelude::Command;
use tmdb_api::tvshow::TVShowShort;
use tmdb_api::tvshow::search::TVShowSearch;

use crate::handler::api::error::ApiError;
use crate::service::database::Database;
use crate::service::tmdb::Tmdb;

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
) -> Result<Json<PaginatedResult<TVShowShort>>, ApiError> {
    let tvshows = TVShowSearch::new(query.query)
        .with_page(Some(query.page + 1))
        .execute(tmdb.as_ref())
        .await?;
    crate::model::tvshow::upsert_all(db.as_ref(), tvshows.results.iter().map(|item| &item.inner))
        .await?;
    Ok(Json(tvshows))
}

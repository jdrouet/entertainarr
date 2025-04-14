use axum::extract::Query;
use axum::{Extension, Json};
use tmdb_api::common::PaginatedResult;
use tmdb_api::movie::MovieShort;
use tmdb_api::movie::search::MovieSearch;
use tmdb_api::prelude::Command;

use crate::handler::api::error::ApiError;
use crate::service::tmdb::Tmdb;

#[derive(Debug, serde::Deserialize)]
pub struct SearchQuery {
    #[serde(alias = "q", default)]
    query: String,
    #[serde(default)]
    page: u32,
}

pub(super) async fn handle(
    Extension(tmdb): Extension<Tmdb>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<PaginatedResult<MovieShort>>, ApiError> {
    MovieSearch::new(query.query)
        .with_page(Some(query.page + 1))
        .execute(tmdb.as_ref())
        .await
        .map(Json)
        .map_err(ApiError::from)
}

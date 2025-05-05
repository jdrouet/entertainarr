use axum::extract::Query;
use axum::{Extension, Json};
use tmdb_api::common::PaginatedResult;
use tmdb_api::movie::MovieShort;

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
    let params = tmdb_api::movie::search::Params::default().with_page(query.page + 1);
    let movies = tmdb.as_ref().search_movies(&query.query, &params).await?;
    Ok(Json(movies))
}

use axum::extract::Query;
use axum::{Extension, Json};
use entertainarr_api::tvshow::TVShow;
use tmdb_api::prelude::Command;
use tmdb_api::tvshow::search::TVShowSearch;

use crate::handler::api::authentication::Authentication;
use crate::handler::api::error::ApiError;
use crate::service::tmdb::Tmdb;
use entertainarr_database::Database;

use super::tvshow_to_view;

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
    Authentication(user_id): Authentication,
) -> Result<Json<Vec<TVShow>>, ApiError> {
    let tvshows = TVShowSearch::new(query.query)
        .with_page(Some(query.page + 1))
        .execute(tmdb.as_ref())
        .await?;
    tracing::debug!("found {} items on tmdb", tvshows.total_results);
    if tvshows.results.is_empty() {
        return Ok(Json(Vec::new()));
    }
    let mut tx = db.as_ref().begin().await?;
    entertainarr_database::model::tvshow::upsert_all(
        &mut *tx,
        tvshows.results.iter().map(|item| &item.inner),
    )
    .await?;
    let list = entertainarr_database::model::tvshow::get_by_ids(
        &mut *tx,
        user_id,
        tvshows.results.iter().map(|item| item.inner.id),
    )
    .await?;
    tx.commit().await?;
    let list = list.into_iter().map(tvshow_to_view).collect::<Vec<_>>();
    Ok(Json(list))
}

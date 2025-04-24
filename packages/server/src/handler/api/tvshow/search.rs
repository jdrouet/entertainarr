use axum::extract::Query;
use axum::{Extension, Json};
use entertainarr_api::tvshow::TVShow;
use entertainarr_database::Database;
use tmdb_api::prelude::Command;
use tmdb_api::tvshow::search::TVShowSearch;

use crate::handler::api::authentication::Authentication;
use crate::handler::api::error::ApiError;
use crate::service::tmdb::Tmdb;

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

#[cfg(test)]
mod tests {
    use mockito::Matcher;

    use crate::service::tmdb::Tmdb;

    #[tokio::test]
    async fn should_search() {
        let mut server = mockito::Server::new_async().await;
        let tmdb = Tmdb::test(server.url());
        let db = entertainarr_database::Config::default()
            .build()
            .await
            .unwrap();
        db.migrate().await.unwrap();

        let search_mock = server
            .mock("GET", "/search/tv")
            .match_query(Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../../../assets/tvshow-search.json"))
            .create();

        let axum::Json(list) = super::handle(
            axum::Extension(db),
            axum::Extension(tmdb),
            axum::extract::Query(super::SearchQuery {
                query: String::from("hello world"),
                page: 0,
            }),
            crate::handler::api::authentication::Authentication(1),
        )
        .await
        .unwrap();

        assert_eq!(list.len(), 4);
        search_mock.assert_async().await;
    }
}

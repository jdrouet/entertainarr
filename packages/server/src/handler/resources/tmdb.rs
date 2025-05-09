use axum::{Extension, body::Body, extract::Path, http::HeaderMap};
use reqwest::StatusCode;

use crate::service::fetcher::Fetcher;

pub(super) async fn handle(
    Extension(fetcher): Extension<Fetcher>,
    Path((size, fname)): Path<(String, String)>,
) -> Result<(HeaderMap, Body), StatusCode> {
    let url = format!("https://image.tmdb.org/t/p/{size}/{fname}");
    fetcher
        .fetch_into_body(&url)
        .await
        .map_err(|err| err.status().unwrap_or(StatusCode::BAD_GATEWAY))
}

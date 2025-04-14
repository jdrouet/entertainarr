use axum::Extension;
use axum::body::Body;
use axum::extract::Path;
use axum::http::header;
use entertainarr_storage::source::AnySource;
use entertainarr_storage::source::prelude::{File, Source};
use tokio_util::io::ReaderStream;

use crate::handler::api::error::ApiError;
use crate::service::storage::Storage;

#[axum::debug_handler]
pub(super) async fn handle(
    Extension(storage): Extension<Storage>,
    Path((name, path)): Path<(String, String)>,
) -> Result<axum::http::Response<Body>, ApiError> {
    let source: &AnySource = storage
        .source(name.as_str())
        .ok_or(ApiError::not_found("source not found"))?;

    let file = source.file(&path).await?;
    let reader = file.reader().await?;
    let stream = ReaderStream::new(reader);
    let body = Body::from_stream(stream);

    Ok(axum::response::Response::builder()
        .header(header::CONTENT_TYPE, file.content_type().essence_str())
        .header(header::CONTENT_LENGTH, file.size())
        .body(body)
        .unwrap())
}

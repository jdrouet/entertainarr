use std::ops::Bound;

use axum::Extension;
use axum::body::Body;
use axum::extract::Path;
use axum::http::header;
use axum_extra::TypedHeader;
use axum_extra::headers::Range;
use entertainarr_storage::source::AnySource;
use entertainarr_storage::source::prelude::{File, Source};
use tokio_util::io::ReaderStream;

use crate::handler::api::error::ApiError;
use crate::service::storage::Storage;

fn content_range(start: Bound<u64>, end: Bound<u64>, size: u64) -> String {
    let start = match start {
        Bound::Unbounded => 0,
        Bound::Included(value) => value,
        Bound::Excluded(value) => value + 1,
    };
    let end = match end {
        Bound::Unbounded => size,
        Bound::Included(value) => value,
        Bound::Excluded(value) => value - 1,
    };
    format!("bytes {start}-{end}/{size}")
}

pub(super) async fn handle(
    Extension(storage): Extension<Storage>,
    Path((name, path)): Path<(String, String)>,
    range: Option<TypedHeader<Range>>,
) -> Result<axum::http::Response<Body>, ApiError> {
    let source: &AnySource = storage
        .source(name.as_str())
        .ok_or(ApiError::not_found("source not found"))?;

    let file = source.file(&path).await?;

    let range = range
        .and_then(|inner| inner.0.satisfiable_ranges(file.size()).next())
        .unwrap_or((Bound::Unbounded, Bound::Unbounded));

    let reader = file.reader(range).await?;
    let stream = ReaderStream::new(reader);
    let body = Body::from_stream(stream);

    Ok(axum::response::Response::builder()
        .header(header::ACCEPT_RANGES, "bytes")
        .header(header::CONTENT_LENGTH, file.size())
        .header(
            header::CONTENT_RANGE,
            content_range(range.0, range.1, file.size()),
        )
        .header(header::CONTENT_TYPE, file.content_type().essence_str())
        .body(body)
        .unwrap())
}

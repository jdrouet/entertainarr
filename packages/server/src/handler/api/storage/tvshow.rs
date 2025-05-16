use std::ops::Bound;
use std::path::PathBuf;

use any_storage::{Store, StoreFile, StoreMetadata};
use axum::Extension;
use axum::body::Body;
use axum::extract::Path;
use axum::http::{HeaderValue, Response, StatusCode};
use axum_extra::TypedHeader;
use axum_extra::headers::Range;
use reqwest::header::CONTENT_TYPE;

use crate::handler::api::error::ApiError;
use crate::service::storage::Storage;

fn from_range((left, right): (Bound<u64>, Bound<u64>), size: u64, max_size: u64) -> (u64, u64) {
    let begin = match left {
        Bound::Excluded(value) => value + 1,
        Bound::Included(value) => value,
        Bound::Unbounded => 0,
    };
    let end = match right {
        Bound::Excluded(value) => value,
        Bound::Included(value) => value + 1,
        Bound::Unbounded => size,
    };
    (begin, (end.min(begin + max_size)))
}

const MAX_RANGE: u64 = 5 * 1024 * 1024; // 5MB

pub(super) async fn handle(
    Extension(storage): Extension<Storage>,
    range: Option<TypedHeader<Range>>,
    Path(path): Path<PathBuf>,
) -> Result<Response<Body>, ApiError> {
    let storage = storage
        .tvshow
        .ok_or_else(|| ApiError::not_found("tvshow storage not configured"))?;
    let file = storage
        .store
        .get_file(&path)
        .await
        .map_err(|err| ApiError::new(StatusCode::BAD_GATEWAY, err.to_string()))?;
    let meta = file
        .metadata()
        .await
        .map_err(|err| ApiError::new(StatusCode::BAD_GATEWAY, err.to_string()))?;
    let (begin, end) = range
        .as_ref()
        .and_then(|TypedHeader(inner)| inner.satisfiable_ranges(meta.size()).next())
        .map(|bound| from_range(bound, meta.size(), MAX_RANGE))
        .unwrap_or((0, MAX_RANGE));
    let reader = file
        .read(begin..end)
        .await
        .map_err(|err| ApiError::new(StatusCode::BAD_GATEWAY, err.to_string()))?;

    let stream = tokio_util::io::ReaderStream::new(reader);
    let mut res = Response::new(Body::from_stream(stream));

    if range.is_some() {
        *res.status_mut() = StatusCode::PARTIAL_CONTENT;
    }

    let headers = res.headers_mut();

    if let Some(content_type) = mime_guess::from_path(&path).first_raw() {
        headers.append(CONTENT_TYPE, HeaderValue::from_static(content_type));
    }

    if range.is_some() {
        headers.append("Accept-Ranges", HeaderValue::from_static("bytes"));

        let value = format!("bytes {begin}-{end}/{}", meta.size());
        headers.append("Content-Range", HeaderValue::from_str(&value).unwrap());
    }

    let value = format!("{}", end - begin - 1);
    headers.append("Content-Length", HeaderValue::from_str(&value).unwrap());

    Ok(res)
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, sync::Arc};

    use any_storage::{any::AnyStore, http::HttpStore};
    use axum::{Extension, extract::Path};
    use axum_extra::{TypedHeader, headers::Range};
    use reqwest::StatusCode;

    use crate::service::storage::{Storage, TVShowStorage};

    const BASE_URL: &str = "https://download.blender.org/peach/bigbuckbunny_movies/";

    #[tokio::test]
    async fn should_handle_with_small_range() {
        let store = AnyStore::Http(HttpStore::new(BASE_URL).unwrap());
        let storage = Storage {
            tvshow: Some(Arc::new(TVShowStorage { store })),
        };
        let response = super::handle(
            Extension(storage),
            Some(TypedHeader(Range::bytes(0..(1024 * 1024)).unwrap())),
            Path(PathBuf::from("big_buck_bunny_1080p_h264.mov")),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::PARTIAL_CONTENT);
        assert_eq!(
            response
                .headers()
                .get("Content-Range")
                .unwrap()
                .to_str()
                .unwrap(),
            "bytes 0-1048576/725106140"
        );
        assert_eq!(
            response
                .headers()
                .get("Content-Length")
                .unwrap()
                .to_str()
                .unwrap(),
            "1048575"
        );
    }

    // should limit to 5MB
    #[tokio::test]
    async fn should_handle_with_big_range() {
        let store = AnyStore::Http(HttpStore::new(BASE_URL).unwrap());
        let storage = Storage {
            tvshow: Some(Arc::new(TVShowStorage { store })),
        };
        let response = super::handle(
            Extension(storage),
            Some(TypedHeader(Range::bytes(0..(100 * 1024 * 1024)).unwrap())),
            Path(PathBuf::from("big_buck_bunny_1080p_h264.mov")),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::PARTIAL_CONTENT);
        assert_eq!(
            response
                .headers()
                .get("Content-Range")
                .unwrap()
                .to_str()
                .unwrap(),
            "bytes 0-5242880/725106140"
        );
        assert_eq!(
            response
                .headers()
                .get("Content-Length")
                .unwrap()
                .to_str()
                .unwrap(),
            "5242879"
        );
    }

    #[tokio::test]
    async fn should_handle_without_range() {
        let store = AnyStore::Http(HttpStore::new(BASE_URL).unwrap());
        let storage = Storage {
            tvshow: Some(Arc::new(TVShowStorage { store })),
        };
        let response = super::handle(
            Extension(storage),
            None,
            Path(PathBuf::from("big_buck_bunny_1080p_h264.mov")),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert!(response.headers().get("Content-Range").is_none());
    }
}

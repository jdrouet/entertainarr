use std::borrow::Cow;

use axum::{
    http::{StatusCode, Uri, header},
    response::{IntoResponse, Response},
    routing::get,
};
use include_dir::Dir;

static WEB_DIR: Dir<'_> = include_dir::include_dir!("$CARGO_MANIFEST_DIR/public");

struct StaticFile(Cow<'static, str>);

impl IntoResponse for StaticFile {
    fn into_response(self) -> Response {
        match WEB_DIR.get_entry(self.0.as_ref()) {
            Some(include_dir::DirEntry::File(file)) => {
                let mime = mime_guess::from_path(file.path()).first_or_octet_stream();
                ([(header::CONTENT_TYPE, mime.as_ref())], file.contents()).into_response()
            }
            _ => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
        }
    }
}

async fn handle_index() -> StaticFile {
    StaticFile("index.html".into())
}

async fn handle_any(uri: Uri) -> StaticFile {
    let uri = Cow::Owned(uri.path().trim_start_matches("/").to_owned());
    StaticFile(uri)
}

pub fn router() -> axum::Router {
    axum::Router::default()
        .route("/assets/{*filepath}", get(handle_any))
        .fallback(handle_index)
}

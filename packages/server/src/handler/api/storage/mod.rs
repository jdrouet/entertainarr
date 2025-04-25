use axum::routing::{get, head, post};

mod list;
mod scan;
mod status;
mod stream;

pub(crate) fn router() -> axum::Router {
    axum::Router::new()
        .route("/scan", post(scan::handle))
        .route("/status", head(status::handle))
        .route("/{name}/list/", get(list::handle_root))
        .route("/{name}/list/{*path}", get(list::handle_path))
        .route("/{name}/stream/{*path}", get(stream::handle))
}

use axum::routing::{get, head};

mod list;
mod status;
mod stream;

pub(crate) fn router() -> axum::Router {
    axum::Router::new()
        .route("/status", head(status::handle))
        .route("/{name}/list/", get(list::handle_root))
        .route("/{name}/list/{*path}", get(list::handle_path))
        .route("/{name}/stream/{*path}", get(stream::handle))
}

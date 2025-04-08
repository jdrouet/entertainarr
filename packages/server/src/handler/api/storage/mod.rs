use axum::routing::{get, head};

mod list;
mod status;

pub(crate) fn router() -> axum::Router {
    axum::Router::new()
        .route("/status", head(status::handle))
        .route("/list/{name}/{*path}", get(list::handle))
}

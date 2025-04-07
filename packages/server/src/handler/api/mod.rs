use axum::routing::head;

mod error;
mod status;

pub(crate) fn router() -> axum::Router {
    axum::Router::new().route("/status", head(status::handle))
}

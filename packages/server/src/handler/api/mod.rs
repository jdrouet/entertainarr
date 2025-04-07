use axum::routing::head;

mod status;

pub(crate) fn router() -> axum::Router {
    axum::Router::new().route("/status", head(status::handle))
}

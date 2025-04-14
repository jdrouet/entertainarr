use axum::routing::get;

mod search;

pub(crate) fn router() -> axum::Router {
    axum::Router::new().route("/search", get(search::handle))
}

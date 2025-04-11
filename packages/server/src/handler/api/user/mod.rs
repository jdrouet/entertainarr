use axum::routing::get;

mod list;

pub(crate) fn router() -> axum::Router {
    axum::Router::new().route("/", get(list::handle))
}

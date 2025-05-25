mod list;

pub(super) fn router() -> axum::Router {
    axum::Router::new().route("/", axum::routing::get(list::handle))
}

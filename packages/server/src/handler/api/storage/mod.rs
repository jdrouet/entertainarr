mod tvshow;

pub(crate) fn router() -> axum::Router {
    axum::Router::new().route("/tvshows/{*path}", axum::routing::get(tvshow::handle))
}

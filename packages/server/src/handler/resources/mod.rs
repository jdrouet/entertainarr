use axum::routing::get;

mod tmdb;

pub(crate) fn router() -> axum::Router {
    axum::Router::default().route("/tmdb/{size}/{fname}", get(tmdb::handle))
}

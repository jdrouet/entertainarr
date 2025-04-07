mod api;

pub(crate) fn router() -> axum::Router {
    axum::Router::new().nest("/api", api::router())
}

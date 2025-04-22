mod api;

pub(crate) fn router() -> axum::Router {
    axum::Router::default().nest("/api", api::router())
}

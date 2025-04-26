mod api;
mod view;

pub(crate) fn router() -> axum::Router {
    axum::Router::default()
        .nest("/api", api::router())
        .merge(view::router())
}

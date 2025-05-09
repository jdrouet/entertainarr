mod api;
mod resources;
mod view;

pub(crate) fn router() -> axum::Router {
    axum::Router::default()
        .nest("/api", api::router())
        .nest("/resources", resources::router())
        .merge(view::router())
}

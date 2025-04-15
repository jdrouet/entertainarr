mod api;
mod view;

pub(crate) fn router() -> axum::Router {
    view::router().nest("/api", api::router())
}

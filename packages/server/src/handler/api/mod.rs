use axum::routing::head;

mod error;
mod movie;
mod status;
mod storage;
mod tvshow;
mod user;

pub(crate) fn router() -> axum::Router {
    axum::Router::new()
        .route("/status", head(status::handle))
        .nest("/files", storage::router())
        .nest("/movies", movie::router())
        .nest("/tvshows", tvshow::router())
        .nest("/users", user::router())
}

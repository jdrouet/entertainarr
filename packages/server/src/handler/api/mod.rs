use axum::routing::head;

mod authentication;
mod error;
mod movie;
mod status;
mod storage;
mod tvshow;
mod user;

async fn not_found() -> error::ApiError {
    error::ApiError::not_found("endpoint not found")
}

pub(crate) fn router() -> axum::Router {
    axum::Router::new()
        .route("/status", head(status::handle))
        .nest("/movies", movie::router())
        .nest("/storages", storage::router())
        .nest("/tvshows", tvshow::router())
        .nest("/users", user::router())
        .fallback(not_found)
}

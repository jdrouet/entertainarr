use axum::routing::head;

mod error;
mod status;
mod storage;
mod user;

pub(crate) fn router() -> axum::Router {
    axum::Router::new()
        .route("/status", head(status::handle))
        .nest("/files", storage::router())
        .nest("/users", user::router())
}

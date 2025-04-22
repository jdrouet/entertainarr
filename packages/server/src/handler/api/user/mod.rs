use axum::routing::{get, post};

mod list;
mod login;

pub(crate) fn router() -> axum::Router {
    axum::Router::new()
        .route("/", get(list::handle))
        .route("/login", post(login::handle))
}

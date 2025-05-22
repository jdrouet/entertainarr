use axum::routing::{get, post};

mod login;
mod me;

pub(crate) fn router() -> axum::Router {
    axum::Router::new()
        .route("/login", post(login::handle))
        .route("/me", get(me::handle))
}

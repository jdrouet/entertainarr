use axum::routing::post;

mod login;

pub(crate) fn router() -> axum::Router {
    axum::Router::new().route("/login", post(login::handle))
}

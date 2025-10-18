use axum::routing::post;

mod login;
mod signup;

pub fn create<S>() -> axum::Router<S>
where
    S: crate::server::prelude::ServerState + Clone,
{
    axum::Router::new()
        .route("/auth/login", post(login::handle::<S>))
        .route("/auth/signup", post(signup::handle::<S>))
}

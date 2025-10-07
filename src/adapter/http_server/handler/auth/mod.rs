use axum::routing::post;

use crate::adapter::http_server::ServerState;

mod login;

pub fn create<AS>() -> axum::Router<ServerState<AS>>
where
    AS: crate::domain::auth::prelude::AuthenticationService + Clone,
{
    axum::Router::new().route("/login", post(login::handle::<AS>))
}

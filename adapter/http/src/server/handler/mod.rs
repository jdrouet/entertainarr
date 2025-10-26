use axum::Json;
use axum::response::IntoResponse;
use axum::routing::{get, head};

mod auth;
pub mod client;
mod podcast;
mod podcast_episode;
pub(crate) mod prelude;
mod status;

pub fn create<S>() -> axum::Router<S>
where
    S: crate::server::prelude::ServerState + Clone,
{
    let api = axum::Router::new()
        .merge(auth::create::<S>())
        .merge(podcast::create::<S>())
        .merge(podcast_episode::create::<S>());

    axum::Router::new()
        .route("/", get(client::handle_index::<S>))
        .route("/{filename}", get(client::handle::<S>))
        .route("/api", head(status::handle))
        .nest("/api", api)
}

impl IntoResponse for crate::entity::ApiError {
    fn into_response(self) -> axum::response::Response {
        (self.status_code, Json(self)).into_response()
    }
}

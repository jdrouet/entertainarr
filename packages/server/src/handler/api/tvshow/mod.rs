use axum::routing::{get, post};

mod follow;
mod get_by_id;
mod search;

pub(crate) fn router() -> axum::Router {
    axum::Router::new()
        .route("/follow", post(follow::create).delete(follow::delete))
        .route("/search", get(search::handle))
        .route("/{tvshow_id}", get(get_by_id::handle))
}

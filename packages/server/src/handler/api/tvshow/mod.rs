use axum::routing::get;

mod get_by_id;
mod search;

pub(crate) fn router() -> axum::Router {
    axum::Router::new()
        .route("/search", get(search::handle))
        .route("/{tvshow_id}", get(get_by_id::handle))
}

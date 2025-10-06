use axum::routing::head;

mod status;

pub fn create() -> axum::Router {
    axum::Router::new().route("/api", head(status::handle))
}

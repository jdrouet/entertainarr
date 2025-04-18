use axum::{
    response::{Html, IntoResponse},
    routing::get,
};

mod error;
mod home;
mod login;
mod storage;
mod watch;

pub(crate) fn router() -> axum::Router {
    axum::Router::new()
        .route("/", get(home::handle))
        .route("/login", get(login::handle))
        .route("/storage/{source}/", get(storage::handle_root))
        .route("/storage/{source}/{*path}", get(storage::handle_path))
        .route("/watch/{source}/{*path}", get(watch::handle))
}

struct View<T>(pub T);

impl<T: entertainarr_web::Template> IntoResponse for View<T> {
    fn into_response(self) -> axum::response::Response {
        Html(self.0.render().unwrap()).into_response()
    }
}

use axum::{
    response::{Html, IntoResponse},
    routing::get,
};

mod authentication;
mod error;
mod home;
mod login;
mod storage;
mod watch;

pub(crate) fn router() -> axum::Router {
    axum::Router::new()
        .route("/", get(home::handle))
        .route("/login", get(login::view).post(login::redirect))
        .route("/storage/{source}/", get(storage::handle_root))
        .route("/storage/{source}/{*path}", get(storage::handle_path))
        .route("/watch/{source}/{*path}", get(watch::handle))
}

struct View(String);

impl<T: entertainarr_web::Template> From<T> for View {
    fn from(value: T) -> Self {
        Self(value.render().unwrap())
    }
}

impl IntoResponse for View {
    fn into_response(self) -> axum::response::Response {
        Html(self.0).into_response()
    }
}

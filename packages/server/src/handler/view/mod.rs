use axum::{
    response::{Html, IntoResponse},
    routing::get,
};

mod error;
mod home;
mod login;

pub(crate) fn router() -> axum::Router {
    axum::Router::new()
        .route("/", get(home::handle))
        .route("/login", get(login::handle))
}

struct View<T>(pub T);

impl<T: entertainarr_web::Template> IntoResponse for View<T> {
    fn into_response(self) -> axum::response::Response {
        Html(self.0.render().unwrap()).into_response()
    }
}

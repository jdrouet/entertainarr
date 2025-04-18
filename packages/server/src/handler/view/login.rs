use axum::{Extension, Form};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use entertainarr_web::login::LoginView;
use entertainarr_web::redirect::RedirectView;

use crate::handler::view::error::ViewError;
use crate::service::database::Database;

impl entertainarr_web::login::User for crate::model::user::Entity {
    fn login_url(&self) -> String {
        format!("/?user={}", self.id)
    }

    fn name(&self) -> &str {
        &self.name
    }
}

pub(super) async fn view() -> Result<super::View, ViewError> {
    Ok(super::View::from(LoginView::default()))
}

#[derive(Debug, serde::Deserialize)]
pub struct Payload {
    name: String,
}

pub(super) async fn redirect(
    Extension(database): Extension<Database>,
    jar: CookieJar,
    Form(payload): Form<Payload>,
) -> Result<(CookieJar, super::View), ViewError> {
    let Some(user) =
        crate::model::user::get_by_name(database.as_ref(), payload.name.as_str()).await?
    else {
        tracing::warn!(message = "user not found");
        return Ok((jar, super::View::from(LoginView::default())));
    };

    let jar = jar.add(Cookie::new("user_id", user.id.to_string()));
    Ok((jar, super::View::from(RedirectView::new("/"))))
}

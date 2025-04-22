use axum::{Extension, Json, http::StatusCode};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};

use crate::{handler::api::error::ApiError, service::database::Database};

pub async fn handle(
    Extension(database): Extension<Database>,
    jar: CookieJar,
    Json(payload): Json<entertainarr_api::user::LoginPayload>,
) -> Result<(CookieJar, StatusCode), ApiError> {
    let user = crate::model::user::get_by_name(database.as_ref(), &payload.username).await?;
    let Some(user) = user else {
        return Err(ApiError::new(StatusCode::BAD_REQUEST, "unknown username"));
    };

    let mut cookie = Cookie::new("user_id", user.id.to_string());
    cookie.set_path("/");
    cookie.set_same_site(SameSite::None);
    cookie.set_secure(true);
    cookie.set_http_only(true);

    let jar = jar.add(cookie);
    Ok((jar, StatusCode::CREATED))
}

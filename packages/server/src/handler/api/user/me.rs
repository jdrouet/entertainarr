use axum::{Extension, Json, http::StatusCode};
use entertainarr_api::user::User;
use entertainarr_database::Database;

use crate::handler::api::{authentication::Authentication, error::ApiError};

pub(super) async fn handle(
    Extension(database): Extension<Database>,
    Authentication(user_id): Authentication,
) -> Result<Json<User>, ApiError> {
    let user = entertainarr_database::model::user::find_by_id(database.as_ref(), user_id).await?;
    user.map(|entity| User {
        id: entity.id,
        name: entity.name,
    })
    .map(Json)
    .ok_or_else(|| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "unknown user id"))
}

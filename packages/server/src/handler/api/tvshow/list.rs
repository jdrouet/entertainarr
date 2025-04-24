use axum::{Extension, Json};
use entertainarr_api::tvshow::TVShow;
use entertainarr_database::Database;

use crate::handler::api::authentication::Authentication;
use crate::handler::api::error::ApiError;

use super::tvshow_to_view;

pub(super) async fn handle(
    Extension(db): Extension<Database>,
    Authentication(user_id): Authentication,
) -> Result<Json<Vec<TVShow>>, ApiError> {
    let list = entertainarr_database::model::tvshow::followed(db.as_ref(), user_id, 0, 50).await?;
    Ok(Json(list.into_iter().map(tvshow_to_view).collect()))
}

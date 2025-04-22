use axum::{Extension, Json};
use tmdb_api::tvshow::TVShowBase;

use crate::handler::api::authentication::Authentication;
use crate::handler::api::error::ApiError;
use crate::service::database::Database;

pub(super) async fn handle(
    Extension(db): Extension<Database>,
    Authentication(user_id): Authentication,
) -> Result<Json<Vec<TVShowBase>>, ApiError> {
    let list = crate::model::tvshow::followed(db.as_ref(), user_id, 0, 50).await?;
    Ok(Json(list))
}

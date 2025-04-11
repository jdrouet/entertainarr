use axum::Extension;
use axum::Json;

use crate::handler::api::error::ApiError;
use crate::service::database::Database;

pub(super) async fn handle(
    Extension(database): Extension<Database>,
) -> Result<Json<Vec<crate::model::user::Entity>>, ApiError> {
    let list = crate::model::user::list(database.as_ref()).await?;
    Ok(Json(list))
}

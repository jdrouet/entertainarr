use axum::{Extension, Json};
use entertainarr_api::{MetaCount, Response, task::Task};
use entertainarr_database::Database;

use crate::handler::api::error::ApiError;

pub(super) async fn handle(
    Extension(_db): Extension<Database>,
) -> Result<Json<Response<Vec<Task>, (), MetaCount>>, ApiError> {
    Ok(Json(Response {
        data: vec![],
        relationships: (),
        meta: MetaCount { count: 0 },
    }))
}

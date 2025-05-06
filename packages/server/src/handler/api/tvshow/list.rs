use axum::{Extension, Json};
use entertainarr_api::tvshow::TVShow;
use entertainarr_api::{MetaCount, Response};
use entertainarr_database::Database;

use crate::handler::api::authentication::Authentication;
use crate::handler::api::error::ApiError;

use super::tvshow_to_view;

type Payload = Response<Vec<TVShow>, (), MetaCount>;

pub(super) async fn handle(
    Extension(db): Extension<Database>,
    Authentication(user_id): Authentication,
) -> Result<Json<Payload>, ApiError> {
    let mut tx = db.as_ref().begin().await?;
    let list = entertainarr_database::model::tvshow::followed(&mut *tx, user_id, 0, 50).await?;
    let count = entertainarr_database::model::tvshow::count_followed(&mut *tx, user_id).await?;
    let data = list.into_iter().map(tvshow_to_view).collect();
    Ok(Json(Response {
        data,
        relationships: (),
        meta: MetaCount { count },
    }))
}

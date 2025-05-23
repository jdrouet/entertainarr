use axum::Extension;
use axum::extract::Path;
use axum::http::StatusCode;
use entertainarr_database::Database;

use crate::handler::api::authentication::Authentication;
use crate::handler::api::error::ApiError;
use crate::service::worker::{Action, Worker};

pub(super) async fn handle(
    Extension(db): Extension<Database>,
    Extension(worker): Extension<Worker>,
    Authentication(user_id): Authentication,
    Path((tvshow_id, season_number, episode_number)): Path<(u64, u64, u64)>,
) -> Result<StatusCode, ApiError> {
    let found = entertainarr_database::model::tvshow_episode::find_by_number(
        db.as_ref(),
        user_id,
        tvshow_id,
        season_number,
        episode_number,
    )
    .await?;
    let found = found.ok_or_else(|| ApiError::not_found("episode not found"))?;

    worker
        .push(Action::transcode_tvshow_episode(found.id))
        .await;

    Ok(StatusCode::CREATED)
}

use axum::Extension;
use axum::extract::Path;
use axum::http::StatusCode;
use entertainarr_database::Database;

use crate::handler::api::error::ApiError;
use crate::service::tmdb::Tmdb;
use crate::service::worker::{Worker, action::Action};

pub(super) async fn single(
    Extension(db): Extension<Database>,
    Extension(tmdb): Extension<Tmdb>,
    Path(tvshow_id): Path<u64>,
) -> Result<StatusCode, ApiError> {
    let mut tx = db.as_ref().begin().await?;
    crate::view::tvshow::synchronize_tvshow(&mut tx, &tmdb, tvshow_id).await?;
    tx.commit().await?;
    Ok(StatusCode::CREATED)
}

pub(super) async fn all(Extension(worker): Extension<Worker>) -> Result<StatusCode, ApiError> {
    worker.push(Action::sync_every_tvshow()).await;
    Ok(StatusCode::CREATED)
}

use any_storage::StoreFile;
use axum::extract::Path;
use axum::{Extension, Json};
use entertainarr_api::file::File;
use entertainarr_api::tvshow_episode::{TVShowEpisode, WithFiles};
use entertainarr_database::Database;

use crate::handler::api::authentication::Authentication;
use crate::handler::api::error::ApiError;
use crate::service::storage::Storage;

use super::episode_to_view;

pub(super) async fn handle(
    Extension(db): Extension<Database>,
    Extension(storage): Extension<Storage>,
    Authentication(user_id): Authentication,
    Path((tvshow_id, season_number, episode_number)): Path<(u64, u64, u64)>,
) -> Result<Json<WithFiles<TVShowEpisode>>, ApiError> {
    let tvshow =
        entertainarr_database::model::tvshow::find_by_id(db.as_ref(), user_id, tvshow_id).await?;
    let tvshow = tvshow.ok_or_else(|| ApiError::not_found("tvshow not found"))?;

    let found = entertainarr_database::model::tvshow_episode::find_by_number(
        db.as_ref(),
        user_id,
        tvshow_id,
        season_number,
        episode_number,
    )
    .await?;
    let found = found.ok_or_else(|| ApiError::not_found("episode not found"))?;

    let files = if let Some(storage) = storage.tvshow {
        match crate::view::tvshow::find_episode_files(
            &storage.store,
            &tvshow,
            season_number,
            episode_number,
        )
        .await
        {
            Ok(found) => found,
            Err(err) => {
                tracing::warn!(message = "something went wrong while searching for episodes", cause = %err);
                Vec::new()
            }
        }
    } else {
        Vec::new()
    };
    let files = files
        .into_iter()
        .map(|file| {
            let path = file.path().to_path_buf();
            let content_type = mime_guess::from_path(&path).first_raw();
            File {
                path,
                content_type: content_type.map(String::from),
            }
        })
        .collect();
    Ok(Json(WithFiles {
        inner: episode_to_view(found),
        files,
    }))
}

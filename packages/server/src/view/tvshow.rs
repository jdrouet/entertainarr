use std::path::PathBuf;

use any_storage::{Store, StoreDirectory, StoreFile};
use chrono::Datelike;
use tokio_stream::StreamExt;

use crate::service::tmdb::Tmdb;

type Transaction<'a> =
    entertainarr_database::sqlx::Transaction<'a, entertainarr_database::sqlx::Sqlite>;

#[tracing::instrument(skip(tx, tmdb))]
pub async fn synchronize_tvshow<'a>(
    tx: &mut Transaction<'a>,
    tmdb: &Tmdb,
    tvshow_id: u64,
) -> Result<(), super::Error> {
    tracing::debug!("fetching details");
    let tvshow = tmdb
        .as_ref()
        .get_tvshow_details(tvshow_id, &Default::default())
        .await?;

    tracing::debug!("storing in database");
    entertainarr_database::model::tvshow::upsert_all(&mut **tx, std::iter::once(&tvshow.inner))
        .await?;

    for season in tvshow.seasons.into_iter() {
        synchronize_tvshow_season(tx, tmdb, tvshow_id, season.inner.season_number).await?;
    }

    tracing::debug!("done");
    Ok(())
}

#[tracing::instrument(skip(tx, tmdb))]
pub async fn synchronize_tvshow_season<'a>(
    tx: &mut Transaction<'a>,
    tmdb: &Tmdb,
    tvshow_id: u64,
    season_number: u64,
) -> Result<(), super::Error> {
    tracing::debug!("fetching details");
    let season = tmdb
        .as_ref()
        .get_tvshow_season_details(tvshow_id, season_number, &Default::default())
        .await?;

    tracing::debug!("storing in database");
    entertainarr_database::model::tvshow_season::upsert_all(
        &mut **tx,
        tvshow_id,
        std::iter::once(&season.inner),
    )
    .await?;

    if !season.episodes.is_empty() {
        entertainarr_database::model::tvshow_episode::upsert_all(
            &mut **tx,
            season.inner.id,
            season.episodes.iter().map(|item| &item.inner),
        )
        .await?;
    }

    tracing::debug!("done");
    Ok(())
}

pub(crate) fn folder_name(tvshow: &entertainarr_database::model::tvshow::Entity) -> String {
    let mut name = limace::Slugifier::default().slugify(&tvshow.name);
    if let Some(date) = tvshow.first_air_date {
        use std::fmt::Write;

        let year = date.year();
        let _ = write!(&mut name, "_{year}");
    }
    name
}

pub(crate) async fn find_episode_files(
    storage: &any_storage::any::AnyStore,
    tvshow: &entertainarr_database::model::tvshow::Entity,
    season: u64,
    episode: u64,
) -> std::io::Result<Vec<any_storage::any::AnyStoreFile>> {
    let base = folder_name(&tvshow);
    let filename = format!("{base}_S{season:0>2}E{episode:0>2}");
    let season_dir = PathBuf::from(base).join(format!("S{season:0>2}"));
    let dir = storage.get_dir(season_dir).await?;
    let reader = dir.read().await?;
    let files = reader
        .filter_map(|e| e.ok())
        .filter_map(|e| e.into_file().ok())
        .filter(|f| {
            f.filename()
                .map(|name| name.starts_with(&filename))
                .unwrap_or(false)
        })
        .collect::<Vec<any_storage::any::AnyStoreFile>>()
        .await;
    Ok(files)
}

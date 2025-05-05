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

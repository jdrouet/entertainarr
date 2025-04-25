use sqlx::{FromRow, QueryBuilder, Row, Sqlite, sqlite::SqliteRow};
use tmdb_api::tvshow::EpisodeShort;

#[derive(Clone, Debug)]
pub struct Entity {
    pub id: u64,
    pub season_id: u64,
    pub name: String,
    pub air_date: Option<chrono::NaiveDate>,
    pub overview: Option<String>,
    pub episode_number: u64,
    pub watch_progress: Option<u64>,
    pub watch_completed: Option<bool>,
}

impl FromRow<'_, SqliteRow> for Entity {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.get(0),
            season_id: row.get(1),
            name: row.get(2),
            air_date: row.get(3),
            overview: row.get(4),
            episode_number: row.get(5),
            watch_progress: row.get(6),
            watch_completed: row.get(7),
        })
    }
}

pub async fn list<'a, X>(
    conn: X,
    user_id: u64,
    tvshow_id: u64,
    season_number: u64,
) -> sqlx::Result<Vec<Entity>>
where
    X: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let sql = r#"SELECT
    tvshow_episodes.id,
    tvshow_episodes.season_id,
    tvshow_episodes.name,
    tvshow_episodes.air_date,
    tvshow_episodes.overview,
    tvshow_episodes.episode_number,
    watched_tvshow_episodes.progress,
    watched_tvshow_episodes.completed
FROM tvshow_episodes
JOIN tvshow_seasons
    ON tvshow_seasons.id = tvshow_episodes.season_id
    AND tvshow_seasons.tvshow_id = ?
    AND tvshow_seasons.season_number = ?
LEFT OUTER JOIN watched_tvshow_episodes
    ON watched_tvshow_episodes.episode_id = tvshow_episodes.id
    AND watched_tvshow_episodes.user_id = ?"#;
    sqlx::query_as(sql)
        .bind(tvshow_id as i64)
        .bind(season_number as i64)
        .bind(user_id as i64)
        .fetch_all(conn)
        .await
}

pub async fn upsert_all<'a, X>(
    conn: X,
    season_id: u64,
    list: impl Iterator<Item = &EpisodeShort>,
) -> sqlx::Result<()>
where
    X: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let mut qb = QueryBuilder::<Sqlite>::new(
        "INSERT INTO tvshow_episodes (id, season_id, name, air_date, overview, episode_number) ",
    );
    qb.push_values(list, |mut b, item| {
        b.push_bind(item.id as i64)
            .push_bind(season_id as i64)
            .push_bind(item.name.as_str())
            .push_bind(item.air_date.as_ref())
            .push_bind(item.overview.as_ref())
            .push_bind(item.episode_number as i64);
    });

    qb.push(
        r#" ON CONFLICT (id)
DO UPDATE SET
    season_id = excluded.season_id,
    name = excluded.name,
    air_date = excluded.air_date,
    overview = excluded.overview,
    episode_number = excluded.episode_number,
    updated_at = CURRENT_TIMESTAMP
"#,
    );

    qb.build().execute(conn).await?;
    Ok(())
}

pub async fn watched<'a, X>(
    conn: X,
    user_id: u64,
    tvshow_id: u64,
    season_number: u64,
    episode_number: u64,
    progress: u64,
    completed: bool,
) -> sqlx::Result<()>
where
    X: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    sqlx::query(
        r#"INSERT INTO watched_tvshow_episodes (user_id, episode_id, progress, completed)
SELECT ?, tvshow_episodes.id, ?, ?
FROM tvshow_episodes
JOIN tvshow_seasons
    ON tvshow_seasons.id = tvshow_episodes.season_id
WHERE tvshow_seasons.tvshow_id = ?
    AND tvshow_seasons.season_number = ?
    AND tvshow_episodes.episode_number = ?
ON CONFLICT DO UPDATE SET
    progress = excluded.progress,
    completed = excluded.completed,
    updated_at = CURRENT_TIMESTAMP"#,
    )
    .bind(user_id as i64)
    .bind(progress as i64)
    .bind(completed)
    .bind(tvshow_id as i64)
    .bind(season_number as i64)
    .bind(episode_number as i64)
    .execute(conn)
    .await?;
    Ok(())
}

pub async fn unwatched<'a, X>(
    conn: X,
    user_id: u64,
    tvshow_id: u64,
    season_number: u64,
    episode_number: u64,
) -> sqlx::Result<()>
where
    X: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    sqlx::query(
        r#"DELETE FROM watched_tvshow_episodes
WHERE user_id = ? AND episode_id IN (
    SELECT tvshow_episodes.id
    FROM tvshow_episodes
    JOIN tvshow_seasons ON tvshow_seasons.id = tvshow_episodes.season_id
    WHERE tvshow_seasons.tvshow_id = ?
        AND tvshow_seasons.season_number = ?
        AND tvshow_episodes.episode_number = ?
)"#,
    )
    .bind(user_id as i64)
    .bind(tvshow_id as i64)
    .bind(season_number as i64)
    .bind(episode_number as i64)
    .execute(conn)
    .await?;
    Ok(())
}

#[derive(Debug)]
pub struct EpisodeSearch {
    pub tvshow_id: u64,
    pub tvshow_name: String,
    pub tvshow_year: Option<u16>,
    pub season_id: u64,
    pub season_number: u16,
    pub episode_id: u64,
    pub episode_number: u16,
}

impl FromRow<'_, SqliteRow> for EpisodeSearch {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            tvshow_id: row.get(0),
            tvshow_name: row.get(1),
            tvshow_year: row.get(2),
            season_id: row.get(3),
            season_number: row.get(4),
            episode_id: row.get(5),
            episode_number: row.get(6),
        })
    }
}

pub async fn search<'a, X>(
    conn: X,
    title: &str,
    _year: Option<u16>,
    season: u16,
    episode: u16,
) -> sqlx::Result<Vec<EpisodeSearch>>
where
    X: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let mut query = QueryBuilder::<Sqlite>::new(
        r#"SELECT
    tvshows.id, tvshows.name, NULL,
    tvshow_seasons.id, tvshow_seasons.season_number,
    tvshow_episodes.id, tvshow_episodes.episode_number
FROM tvshows
JOIN tvshow_seasons ON tvshows.id = tvshow_seasons.tvshow_id
JOIN tvshow_episodes ON tvshow_seasons.id = tvshow_episodes.season_id"#,
    );
    query
        .push(" WHERE tvshow_seasons.season_number = ")
        .push_bind(season);
    query
        .push(" AND tvshow_episodes.episode_number = ")
        .push_bind(episode);
    query.push(" AND ( true");
    for word in title.split(' ') {
        query
            .push(" AND lower(tvshows.name) LIKE ")
            .push_bind(format!("%{word}%"));
    }
    query.push(" ) LIMIT 10");
    tracing::info!(
        message = query.sql(),
        season = season,
        episode = episode,
        title = title
    );
    query
        .build_query_as::<EpisodeSearch>()
        .fetch_all(conn)
        .await
}

#[cfg(test)]
pub fn build_episode(
    season_number: u64,
    episode_id: u64,
    episode_number: u64,
) -> tmdb_api::tvshow::EpisodeShort {
    tmdb_api::tvshow::EpisodeShort {
        id: episode_id,
        name: format!("Episode Name #{episode_number}"),
        air_date: None,
        overview: Some("A test show".to_string()),
        episode_number,
        production_code: "Whatever".into(),
        season_number,
        still_path: None,
        vote_average: 4.5,
        vote_count: 42,
    }
}

#[cfg(test)]
pub async fn create_episode(
    db: &crate::Database,
    season_id: u64,
    season_number: u64,
    episode_id: u64,
    episode_number: u64,
) {
    let episode = build_episode(season_number, episode_id, episode_number);
    upsert_all(db.as_ref(), season_id, std::iter::once(&episode))
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn should_list_episodes() -> Result<(), sqlx::Error> {
        let db = crate::init().await;
        crate::model::user::create_user(1, "alice")
            .persist(db.as_ref())
            .await?;
        crate::model::user::create_user(2, "bob")
            .persist(db.as_ref())
            .await?;

        crate::model::tvshow::create_tvshow(&db, 1).await;
        crate::model::tvshow_season::create_season(&db, 1, 1, 1).await;
        super::create_episode(&db, 1, 1, 1, 1).await;
        super::create_episode(&db, 1, 1, 2, 2).await;
        super::create_episode(&db, 1, 1, 3, 3).await;
        crate::model::tvshow_season::create_season(&db, 1, 2, 2).await;
        super::create_episode(&db, 2, 2, 4, 1).await;
        super::create_episode(&db, 2, 2, 5, 2).await;

        let list = super::list(db.as_ref(), 1, 1, 1).await?;
        assert_eq!(list.len(), 3);

        let list = super::list(db.as_ref(), 1, 1, 2).await?;
        assert_eq!(list.len(), 2);

        let list = super::list(db.as_ref(), 1, 2, 1).await?;
        assert_eq!(list.len(), 0);

        Ok(())
    }

    #[tokio::test]
    async fn should_mark_as_watched() -> Result<(), sqlx::Error> {
        let db = crate::init().await;
        crate::model::user::create_user(1, "alice")
            .persist(db.as_ref())
            .await?;
        crate::model::user::create_user(2, "bob")
            .persist(db.as_ref())
            .await?;

        crate::model::tvshow::create_tvshow(&db, 1).await;
        crate::model::tvshow_season::create_season(&db, 1, 1, 1).await;
        super::create_episode(&db, 1, 1, 1, 1).await;
        super::create_episode(&db, 1, 1, 2, 2).await;

        super::watched(db.as_ref(), 1, 1, 1, 1, 10, false).await?;
        super::watched(db.as_ref(), 1, 1, 1, 2, 10, true).await?;

        let list = super::list(db.as_ref(), 1, 1, 1).await?;
        assert_eq!(list.len(), 2);

        Ok(())
    }
}

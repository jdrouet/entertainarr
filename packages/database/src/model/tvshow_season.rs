use sqlx::{FromRow, QueryBuilder, Row, Sqlite, sqlite::SqliteRow};
use tmdb_api::tvshow::SeasonBase;

const BASE_QUERY: &str = r#"WITH tvshow_episodes_subset AS (
SELECT tvshow_episodes.*
FROM tvshow_episodes
JOIN tvshow_seasons
    ON tvshow_seasons.id = tvshow_episodes.season_id
    AND tvshow_seasons.tvshow_id = ?
), tvshow_season_episode_count AS (
SELECT
    tvshow_episodes_subset.season_id AS season_id,
    count(tvshow_episodes_subset.id) AS episode_count
FROM tvshow_episodes_subset
GROUP BY tvshow_episodes_subset.season_id
), tvshow_season_episode_seen AS (
SELECT
    tvshow_episodes_subset.season_id AS season_id,
    count(tvshow_episodes_subset.id) AS episode_count
FROM tvshow_episodes_subset
JOIN watched_tvshow_episodes
    ON watched_tvshow_episodes.episode_id = tvshow_episodes_subset.id
    AND watched_tvshow_episodes.user_id = ?
WHERE watched_tvshow_episodes.completed
GROUP BY tvshow_episodes_subset.season_id
) SELECT
tvshow_seasons.id,
tvshow_seasons.tvshow_id,
tvshow_seasons.name,
tvshow_seasons.air_date,
tvshow_seasons.overview,
tvshow_seasons.poster_path,
tvshow_seasons.season_number,
ifnull(tvshow_season_episode_count.episode_count, 0),
ifnull(tvshow_season_episode_seen.episode_count, 0)
FROM tvshow_seasons
LEFT OUTER JOIN tvshow_season_episode_count
ON tvshow_season_episode_count.season_id = tvshow_seasons.id
LEFT OUTER JOIN tvshow_season_episode_seen
ON tvshow_season_episode_seen.season_id = tvshow_seasons.id
WHERE tvshow_id = ?"#;

#[derive(Clone, Debug)]
pub struct Entity {
    pub id: u64,
    pub tvshow_id: u64,
    pub name: String,
    pub air_date: Option<chrono::NaiveDate>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub season_number: u64,
    pub episode_count: u32,
    pub watched_episode_count: u32,
}

impl FromRow<'_, SqliteRow> for Entity {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.get(0),
            tvshow_id: row.get(1),
            name: row.get(2),
            air_date: row.get(3),
            overview: row.get(4),
            poster_path: row.get(5),
            season_number: row.get(6),
            episode_count: row.get(7),
            watched_episode_count: row.get(8),
        })
    }
}

pub async fn find_by_number<'a, X>(
    conn: X,
    user_id: u64,
    tvshow_id: u64,
    season_number: u64,
) -> sqlx::Result<Option<Entity>>
where
    X: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let sql = constcat::concat!(BASE_QUERY, " AND season_number = ? LIMIT 1");
    sqlx::query_as(sql)
        .bind(tvshow_id as i64)
        .bind(user_id as i64)
        .bind(tvshow_id as i64)
        .bind(season_number as i64)
        .fetch_optional(conn)
        .await
}

pub async fn list<'a, X>(conn: X, user_id: u64, tvshow_id: u64) -> sqlx::Result<Vec<Entity>>
where
    X: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let sql = constcat::concat!(BASE_QUERY, "ORDER BY tvshow_seasons.season_number");
    sqlx::query_as(sql)
        .bind(tvshow_id as i64)
        .bind(user_id as i64)
        .bind(tvshow_id as i64)
        .fetch_all(conn)
        .await
}

pub async fn upsert_all<'a, X>(
    conn: X,
    tvshow_id: u64,
    list: impl Iterator<Item = &SeasonBase>,
) -> sqlx::Result<()>
where
    X: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let mut qb = QueryBuilder::<Sqlite>::new(
        "INSERT INTO tvshow_seasons (id, tvshow_id, name, air_date, overview, poster_path, season_number) ",
    );
    qb.push_values(list, |mut b, item| {
        b.push_bind(item.id as i64)
            .push_bind(tvshow_id as i64)
            .push_bind(item.name.as_str())
            .push_bind(item.air_date.as_ref())
            .push_bind(item.overview.as_ref())
            .push_bind(item.poster_path.as_ref())
            .push_bind(item.season_number as i64);
    });

    qb.push(
        r#" ON CONFLICT (id)
DO UPDATE SET
    tvshow_id = excluded.tvshow_id,
    name = excluded.name,
    air_date = excluded.air_date,
    overview = excluded.overview,
    poster_path = excluded.poster_path,
    season_number = excluded.season_number,
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
) -> sqlx::Result<()>
where
    X: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    sqlx::query(
        r#"INSERT INTO watched_tvshow_episodes (user_id, episode_id, progress, completed)
SELECT ?, tvshow_episodes.id, 0, true
FROM tvshow_episodes
JOIN tvshow_seasons
    ON tvshow_seasons.id = tvshow_episodes.season_id
WHERE tvshow_seasons.tvshow_id = ?
    AND tvshow_seasons.season_number = ?
ON CONFLICT DO UPDATE SET
    progress = excluded.progress,
    completed = excluded.completed,
    updated_at = CURRENT_TIMESTAMP"#,
    )
    .bind(user_id as i64)
    .bind(tvshow_id as i64)
    .bind(season_number as i64)
    .execute(conn)
    .await?;
    Ok(())
}

pub async fn unwatched<'a, X>(
    conn: X,
    user_id: u64,
    tvshow_id: u64,
    season_number: u64,
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
)"#,
    )
    .bind(user_id as i64)
    .bind(tvshow_id as i64)
    .bind(season_number as i64)
    .execute(conn)
    .await?;
    Ok(())
}

#[cfg(test)]
pub fn build_season(id: u64, season_number: u64) -> tmdb_api::tvshow::SeasonBase {
    tmdb_api::tvshow::SeasonBase {
        id,
        name: format!("Season Name #{season_number}"),
        air_date: None,
        overview: Some("A test show".to_string()),
        poster_path: Some("/poster.jpg".to_string()),
        season_number,
    }
}

#[cfg(test)]
pub async fn create_season(
    db: &crate::Database,
    tvshow_id: u64,
    season_id: u64,
    season_number: u64,
) {
    let season = build_season(season_id, season_number);
    upsert_all(db.as_ref(), tvshow_id, std::iter::once(&season))
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn should_list_tvshow_seasons() -> Result<(), sqlx::Error> {
        let db = crate::init().await;
        crate::model::user::create_user(1, "alice")
            .persist(db.as_ref())
            .await?;
        crate::model::user::create_user(2, "bob")
            .persist(db.as_ref())
            .await?;

        crate::model::tvshow::create_tvshow(&db, 1).await;
        super::create_season(&db, 1, 1, 1).await;
        super::create_season(&db, 1, 2, 2).await;

        crate::model::tvshow::create_tvshow(&db, 2).await;
        super::create_season(&db, 2, 3, 1).await;

        let list = super::list(db.as_ref(), 1, 1).await?;
        assert_eq!(list.len(), 2);

        let list = super::list(db.as_ref(), 1, 2).await?;
        assert_eq!(list.len(), 1);

        let list = super::list(db.as_ref(), 1, 3).await?;
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
        crate::model::tvshow_episode::create_episode(&db, 1, 1, 1, 1).await;
        crate::model::tvshow_episode::create_episode(&db, 1, 1, 2, 2).await;
        crate::model::tvshow_episode::create_episode(&db, 1, 1, 3, 3).await;

        crate::model::tvshow_season::create_season(&db, 1, 2, 2).await;
        crate::model::tvshow_episode::create_episode(&db, 2, 2, 4, 1).await;
        crate::model::tvshow_episode::create_episode(&db, 2, 2, 5, 2).await;

        crate::model::tvshow_episode::watched(db.as_ref(), 1, 1, 1, 1, 100, true).await?;
        crate::model::tvshow_episode::watched(db.as_ref(), 1, 1, 1, 2, 10, false).await?;

        let list = super::list(db.as_ref(), 1, 1).await?;
        assert_eq!(list.len(), 2);

        let season = list.first().unwrap();
        assert_eq!(season.episode_count, 3);
        assert_eq!(
            season.watched_episode_count, 1,
            "should have watched a single episode"
        );

        Ok(())
    }
}

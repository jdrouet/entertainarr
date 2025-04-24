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
        })
    }
}

pub async fn list<'a, X>(conn: X, tvshow_id: u64, season_number: u64) -> sqlx::Result<Vec<Entity>>
where
    X: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let sql = r#"SELECT tvshow_episodes.id, tvshow_episodes.season_id, tvshow_episodes.name, tvshow_episodes.air_date, tvshow_episodes.overview, tvshow_episodes.episode_number
FROM tvshow_episodes
JOIN tvshow_seasons
    ON tvshow_seasons.id = tvshow_episodes.season_id
    AND tvshow_seasons.tvshow_id = ?
    AND tvshow_seasons.season_number = ?"#;
    sqlx::query_as(sql)
        .bind(tvshow_id as i64)
        .bind(season_number as i64)
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
            .push_bind(item.season_number as i64);
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

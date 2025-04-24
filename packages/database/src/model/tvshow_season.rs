use sqlx::{FromRow, QueryBuilder, Row, Sqlite, sqlite::SqliteRow};
use tmdb_api::tvshow::SeasonBase;

#[derive(Clone, Debug)]
pub struct Entity {
    pub id: u64,
    pub tvshow_id: u64,
    pub name: String,
    pub air_date: Option<chrono::NaiveDate>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub season_number: u64,
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
        })
    }
}

pub async fn list<'a, X>(conn: X, tvshow_id: u64) -> sqlx::Result<Vec<Entity>>
where
    X: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let sql = "SELECT id, tvshow_id, name, air_date, overview, poster_path, season_number FROM tvshow_seasons WHERE tvshow_id = ?";
    sqlx::query_as(sql)
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

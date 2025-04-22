use sqlx::{FromRow, QueryBuilder, Row, Sqlite, sqlite::SqliteRow};
use tmdb_api::tvshow::TVShowBase;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Entity(TVShowBase);

impl FromRow<'_, SqliteRow> for Entity {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self(TVShowBase {
            id: row.get(0),
            name: row.get(1),
            original_name: row.get(2),
            original_language: row.get(3),
            origin_country: {
                let field: String = row.get(4);
                field
                    .split(',')
                    .map(|item| item.trim().to_owned())
                    .collect::<Vec<_>>()
            },
            overview: row.get(5),
            first_air_date: None, // row.get(6)
            poster_path: row.get(7),
            backdrop_path: row.get(8),
            popularity: row.get(9),
            vote_count: row.get(10),
            vote_average: row.get(11),
            adult: row.get(12),
        }))
    }
}

pub async fn find_by_id<'a, X>(conn: X, tvshow_id: u64) -> sqlx::Result<Option<TVShowBase>>
where
    X: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let sql = "SELECT id, name, original_name, original_language, origin_country, overview, first_air_date, poster_path, backdrop_path, popularity, vote_count, vote_average, adult FROM tvshows WHERE id = ? LIMIT 1";
    let entity: Option<Entity> = sqlx::query_as(sql)
        .bind(tvshow_id as i64)
        .fetch_optional(conn)
        .await?;
    Ok(entity.map(|inner| inner.0))
}

pub async fn upsert_all<'a, X>(conn: X, list: impl Iterator<Item = &TVShowBase>) -> sqlx::Result<()>
where
    X: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let mut qb = QueryBuilder::<Sqlite>::new(
        "INSERT INTO tvshows (id, name, original_name, original_language, origin_country, overview, first_air_date, poster_path, backdrop_path, popularity, vote_count, vote_average, adult) ",
    );
    qb.push_values(list, |mut b, item| {
        b.push_bind(item.id as i64)
            .push_bind(item.name.as_str())
            .push_bind(item.original_name.as_str())
            .push_bind(item.original_language.as_str())
            .push_bind(item.origin_country.join(","))
            .push_bind(item.overview.as_ref())
            .push_bind(Some(0))
            .push_bind(item.poster_path.as_ref())
            .push_bind(item.backdrop_path.as_ref())
            .push_bind(item.popularity)
            .push_bind(item.vote_count as i64)
            .push_bind(item.vote_average)
            .push_bind(item.adult);
    });

    qb.push(
        r#" ON CONFLICT (id)
DO UPDATE SET
    name = excluded.name,
    original_name = excluded.original_name,
    original_language = excluded.original_language,
    origin_country = excluded.origin_country,
    overview = excluded.overview,
    first_air_date = excluded.first_air_date,
    poster_path = excluded.poster_path,
    backdrop_path = excluded.backdrop_path,
    popularity = excluded.popularity,
    vote_count = excluded.vote_count,
    vote_average = excluded.vote_average,
    adult = excluded.adult,
    updated_at = CURRENT_TIMESTAMP
"#,
    );

    qb.build().execute(conn).await?;
    Ok(())
}

pub async fn follow<'a, X>(conn: X, user_id: u64, tvshow_id: u64) -> sqlx::Result<()>
where
    X: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    sqlx::query(
        r#"INSERT INTO followed_tvshows (user_id, tvshow_id) VALUES (?,?) ON CONFLICT DO NOTHING"#,
    )
    .bind(user_id as i64)
    .bind(tvshow_id as i64)
    .execute(conn)
    .await?;
    Ok(())
}

pub async fn unfollow<'a, X>(conn: X, user_id: u64, tvshow_id: u64) -> sqlx::Result<()>
where
    X: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    sqlx::query(r#"DELETE FROM followed_tvshows WHERE user_id = ? AND tvshow_id = ?"#)
        .bind(user_id as i64)
        .bind(tvshow_id as i64)
        .execute(conn)
        .await?;
    Ok(())
}

pub async fn followed<'a, X>(
    conn: X,
    user_id: u64,
    offset: u32,
    count: u32,
) -> sqlx::Result<Vec<TVShowBase>>
where
    X: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let list: Vec<Entity> = sqlx::query_as(r#"SELECT id, name, original_name, original_language, origin_country, overview, first_air_date, poster_path, backdrop_path, popularity, vote_count, vote_average, adult
FROM tvshows
WHERE id IN (SELECT tvshow_id FROM followed_tvshows WHERE user_id = ?)
LIMIT ? OFFSET ?"#)
        .bind(user_id as i64)
        .bind(count)
        .bind(offset)
        .fetch_all(conn)
        .await?;
    Ok(list.into_iter().map(|item| item.0).collect())
}

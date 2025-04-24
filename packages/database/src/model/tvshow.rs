use sqlx::{FromRow, QueryBuilder, Row, Sqlite, sqlite::SqliteRow};
use tmdb_api::tvshow::TVShowBase;

// with left outer join
const BASE_VIEW: &str = r#"SELECT
    tvshows.id, tvshows.name, tvshows.original_name, tvshows.original_language, tvshows.origin_country, tvshows.overview, tvshows.first_air_date, tvshows.poster_path, tvshows.backdrop_path, tvshows.popularity, tvshows.vote_count, tvshows.vote_average, tvshows.adult, followed_tvshows.created_at is not null
FROM tvshows
LEFT OUTER JOIN followed_tvshows ON tvshows.id = followed_tvshows.tvshow_id"#;

// with an inner join, won't show for unfollowed
const FOLLOWED_VIEW: &str = r#"SELECT
    tvshows.id, tvshows.name, tvshows.original_name, tvshows.original_language, tvshows.origin_country, tvshows.overview, tvshows.first_air_date, tvshows.poster_path, tvshows.backdrop_path, tvshows.popularity, tvshows.vote_count, tvshows.vote_average, tvshows.adult, followed_tvshows.created_at is not null
FROM tvshows
JOIN followed_tvshows ON tvshows.id = followed_tvshows.tvshow_id AND followed_tvshows.user_id = ?"#;

#[derive(Clone, Debug)]
pub struct Entity {
    pub id: u64,
    pub name: String,
    pub original_name: String,
    pub original_language: String,
    pub origin_country: Vec<String>,
    pub overview: Option<String>,
    pub first_air_date: Option<chrono::NaiveDate>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub popularity: f64,
    pub vote_count: u64,
    pub vote_average: f64,
    pub adult: bool,
    pub following: bool,
}

impl FromRow<'_, SqliteRow> for Entity {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
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
            first_air_date: row.get(6),
            poster_path: row.get(7),
            backdrop_path: row.get(8),
            popularity: row.get(9),
            vote_count: row.get(10),
            vote_average: row.get(11),
            adult: row.get(12),
            following: row.get(13),
        })
    }
}

const FIND_BY_ID_SQL: &str = constcat::concat!(
    BASE_VIEW,
    " AND followed_tvshows.user_id = ? WHERE tvshows.id = ? LIMIT 1"
);

pub async fn find_by_id<'a, X>(
    conn: X,
    user_id: u64,
    tvshow_id: u64,
) -> sqlx::Result<Option<Entity>>
where
    X: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    sqlx::query_as(FIND_BY_ID_SQL)
        .bind(user_id as i64)
        .bind(tvshow_id as i64)
        .fetch_optional(conn)
        .await
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
            .push_bind(item.first_air_date.as_ref())
            .push_bind(item.poster_path.as_ref())
            .push_bind(item.backdrop_path.as_ref())
            .push_bind(item.popularity)
            .push_bind(item.vote_count as i64)
            .push_bind(item.vote_average)
            .push_bind(item.adult);
    });

    qb.push(
        r#" ON CONFLICT (id) DO UPDATE SET
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
    updated_at = CURRENT_TIMESTAMP"#,
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

pub async fn get_by_ids<'a, X>(
    conn: X,
    user_id: u64,
    tvshow_ids: impl Iterator<Item = u64>,
) -> sqlx::Result<Vec<Entity>>
where
    X: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let mut qb = QueryBuilder::<Sqlite>::new(BASE_VIEW);
    qb.push(" AND followed_tvshows.user_id =")
        .push_bind(user_id as i64);
    qb.push(" WHERE false ");
    for id in tvshow_ids {
        qb.push("OR tvshows.id = ").push_bind(id as i64);
    }
    qb.build_query_as::<Entity>().fetch_all(conn).await
}

const FOLLOWED_SQL: &str = constcat::concat!(FOLLOWED_VIEW, " LIMIT ? OFFSET ?");

pub async fn followed<'a, X>(
    conn: X,
    user_id: u64,
    offset: u32,
    count: u32,
) -> sqlx::Result<Vec<Entity>>
where
    X: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    sqlx::query_as(FOLLOWED_SQL)
        .bind(user_id as i64)
        .bind(count)
        .bind(offset)
        .fetch_all(conn)
        .await
}

#[cfg(test)]
pub fn build_tvshow(id: u64) -> tmdb_api::tvshow::TVShowBase {
    use chrono::NaiveDate;

    tmdb_api::tvshow::TVShowBase {
        id,
        name: "Test Show".to_string(),
        original_name: "Test Original".to_string(),
        original_language: "en".to_string(),
        origin_country: vec!["US".to_string()],
        overview: Some("A test show".to_string()),
        first_air_date: Some(NaiveDate::from_ymd_opt(2022, 1, 1).unwrap()),
        poster_path: Some("/poster.jpg".to_string()),
        backdrop_path: Some("/backdrop.jpg".to_string()),
        popularity: 100.0,
        vote_count: 123,
        vote_average: 8.5,
        adult: false,
    }
}

#[cfg(test)]
pub async fn create_tvshow(db: &crate::Database, id: u64) {
    let show = build_tvshow(id);
    upsert_all(db.as_ref(), std::iter::once(&show))
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use tmdb_api::tvshow::TVShowBase;

    async fn setup_db() -> crate::Database {
        let db = crate::init().await;
        crate::model::user::create_user(1, "alice")
            .persist(db.as_ref())
            .await
            .unwrap();
        crate::model::user::create_user(2, "bob")
            .persist(db.as_ref())
            .await
            .unwrap();
        db
    }

    fn sample_show(id: u64) -> TVShowBase {
        TVShowBase {
            id,
            name: "Test Show".to_string(),
            original_name: "Test Original".to_string(),
            original_language: "en".to_string(),
            origin_country: vec!["US".to_string()],
            overview: Some("A test show".to_string()),
            first_air_date: Some(NaiveDate::from_ymd_opt(2022, 1, 1).unwrap()),
            poster_path: Some("/poster.jpg".to_string()),
            backdrop_path: Some("/backdrop.jpg".to_string()),
            popularity: 100.0,
            vote_count: 123,
            vote_average: 8.5,
            adult: false,
        }
    }

    #[tokio::test]
    async fn test_upsert_and_find_by_id() {
        crate::enable_tracing();
        let db = setup_db().await;

        let show = super::build_tvshow(1);
        super::upsert_all(db.as_ref(), std::iter::once(&show))
            .await
            .unwrap();

        let found = super::find_by_id(db.as_ref(), 1, 1).await.unwrap().unwrap();
        assert_eq!(found.id, 1);
        assert_eq!(found.name, "Test Show");
        assert!(!found.following);
    }

    #[tokio::test]
    async fn test_follow_and_unfollow() {
        let db = setup_db().await;
        let user_id = 1;
        let show = sample_show(2);
        super::upsert_all(db.as_ref(), std::iter::once(&show))
            .await
            .unwrap();

        super::follow(db.as_ref(), user_id, 2).await.unwrap();
        let followed = super::find_by_id(db.as_ref(), user_id, 2)
            .await
            .unwrap()
            .unwrap();
        assert!(followed.following);

        super::unfollow(db.as_ref(), user_id, 2).await.unwrap();
        let unfollowed = super::find_by_id(db.as_ref(), user_id, 2)
            .await
            .unwrap()
            .unwrap();
        assert!(!unfollowed.following);
    }

    #[tokio::test]
    async fn test_followed_list_pagination() {
        let db = setup_db().await;
        let user_id = 1;
        let other_user_id = 2;
        let shows: Vec<_> = (1..6).map(sample_show).collect();
        super::upsert_all(db.as_ref(), shows.iter()).await.unwrap();

        let fetched = super::get_by_ids(db.as_ref(), user_id, 1..6).await.unwrap();
        assert_eq!(fetched.len(), shows.len());

        for show in &shows {
            super::follow(db.as_ref(), user_id, show.id).await.unwrap();
        }

        let first_page = super::followed(db.as_ref(), user_id, 0, 3).await.unwrap();
        assert_eq!(first_page.len(), 3);

        let other_user = super::followed(db.as_ref(), other_user_id, 0, 3)
            .await
            .unwrap();
        assert!(other_user.is_empty());

        let second_page = super::followed(db.as_ref(), user_id, 3, 3).await.unwrap();
        assert_eq!(second_page.len(), 2);
    }
}

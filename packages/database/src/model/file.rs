use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use sqlx::{FromRow, Row, sqlite::SqliteRow};

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
pub enum Store {
    #[serde(rename = "tvshow")]
    TVShow,
}

impl Store {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::TVShow => "tvshow",
        }
    }

    fn decode(value: String) -> sqlx::Result<Self> {
        match value.as_str() {
            "tvshow" => Ok(Self::TVShow),
            other => Err(sqlx::Error::Decode(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("unable to decode {other:?} into `Store`"),
            )))),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Entity {
    pub id: u64,
    pub store: Store,
    pub path: PathBuf,
    pub size: u64,
    pub content_type: Option<String>,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

impl FromRow<'_, SqliteRow> for Entity {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.get(0),
            store: row.try_get::<'_, String, _>(1).and_then(Store::decode)?,
            path: PathBuf::from(row.get::<'_, String, _>(2)),
            size: row.get(3),
            content_type: row.get(4),
            created_at: row.get(5),
            modified_at: row.get(6),
        })
    }
}

pub async fn upsert<'a, X>(
    conn: X,
    store: Store,
    path: &Path,
    size: u64,
    content_type: Option<&str>,
    created_at: u64,
    modified_at: u64,
) -> sqlx::Result<()>
where
    X: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let query = r#"INSERT INTO files (store, path, size, content_type, created_at, modified_at)
VALUES (?, ?, ?, ?, ?, ?)
ON CONFLICT (store, path)
DO UPDATE SET size = excluded.size, content_type = excluded.content_type, created_at = excluded.created_at, modified_at = excluded.modified_at
RETURNING id"#;
    sqlx::query(query)
        .persistent(true)
        .bind(store.as_str())
        .bind(&path.to_string_lossy())
        .bind(size as i64)
        .bind(content_type)
        .bind(created_at as i64)
        .bind(modified_at as i64)
        .fetch_one(conn)
        .await?;
    Ok(())
}

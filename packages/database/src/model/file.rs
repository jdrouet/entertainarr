use sqlx::{FromRow, Row, sqlite::SqliteRow};

#[derive(Debug)]
pub struct Entity {
    pub id: u64,
    pub source: String,
    pub directory: String,
    pub filename: String,
    pub size: u64,
    pub created_at: u64,
    pub modified_at: u64,
}

impl FromRow<'_, SqliteRow> for Entity {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.get(0),
            source: row.get(1),
            directory: row.get(2),
            filename: row.get(3),
            size: row.get(4),
            created_at: row.get(5),
            modified_at: row.get(6),
        })
    }
}

pub async fn upsert<'a, X>(conn: X, item: &Entity) -> sqlx::Result<Entity>
where
    X: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let sql = r#"INSERT INTO files (source, directory, filename, size, created_at, modified_at)
VALUES (?,?,?,?,?,?)
ON CONFLICT (source, directory, filename) DO UPDATE SET
    modified_at = excluded.modified_at
RETURNING id, source, directory, filename, size, created_at, modified_at"#;
    sqlx::query_as(sql)
        .bind(&item.source)
        .bind(&item.directory)
        .bind(&item.filename)
        .bind(item.size as i64)
        .bind(item.created_at as i64)
        .bind(item.modified_at as i64)
        .fetch_one(conn)
        .await
}

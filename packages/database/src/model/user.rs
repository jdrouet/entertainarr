use sqlx::{FromRow, Row, sqlite::SqliteRow};

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Entity {
    pub id: u64,
    pub name: Box<str>,
}

impl FromRow<'_, SqliteRow> for Entity {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.get(0),
            name: row.get(1),
        })
    }
}

impl Entity {
    pub async fn persist<'a, X>(&self, conn: X) -> sqlx::Result<()>
    where
        X: sqlx::Executor<'a, Database = sqlx::Sqlite>,
    {
        sqlx::query_as(r#"INSERT INTO users (id, name) VALUES (?, ?) ON CONFLICT DO UPDATE SET name = excluded.name RETURNING id, name"#)
            .bind(self.id as i64)
            .bind(&self.name)
            .fetch_one(conn)
            .await
    }
}

pub async fn list<'a, X>(conn: X) -> sqlx::Result<Vec<Entity>>
where
    X: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    sqlx::query_as(r#"SELECT id, name FROM users ORDER BY name"#)
        .fetch_all(conn)
        .await
}

pub async fn get_by_name<'a, X>(conn: X, name: &str) -> sqlx::Result<Option<Entity>>
where
    X: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    sqlx::query_as(r#"SELECT id, name FROM users WHERE name = ? LIMIT 1"#)
        .bind(name)
        .fetch_optional(conn)
        .await
}

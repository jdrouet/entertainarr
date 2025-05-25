use chrono::{DateTime, Utc};
use sqlx::sqlite::SqliteRow;
use sqlx::types::Json;
use sqlx::{FromRow, Row};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Task {
    pub id: u64,
    pub action: Action,
    pub status: Status,
    pub retry: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl FromRow<'_, SqliteRow> for Task {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.get(0),
            action: row.try_get(1).map(|Json(value)| value)?,
            status: row.try_get(2).map(|Json(value)| value)?,
            retry: row.try_get(3)?,
            created_at: row.try_get(4)?,
            updated_at: row.try_get(5)?,
        })
    }
}

impl Task {
    pub async fn find_last<'a, X>(&self, conn: X, action: Action) -> sqlx::Result<Option<Self>>
    where
        X: sqlx::Executor<'a, Database = sqlx::Sqlite>,
    {
        let sql = r#"SELECT id, action, status, retry, created_at, updated_at FROM tasks WHERE action = ? LIMIT 1"#;
        sqlx::query_as(sql)
            .bind(Json(action))
            .fetch_optional(conn)
            .await
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Action {
    SynchronizeEveryTVShow(SynchronizeEveryTVShow),
    SynchronizeTVShow(SynchronizeTVShow),
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SynchronizeEveryTVShow;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SynchronizeTVShow {
    pub tvshow_id: u64,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Status {
    Waiting,
    Progress,
    Canceled,
    Completed,
    Failed,
}

pub struct CreateTask {
    action: Action,
    status: Status,
}

impl CreateTask {
    pub fn new(action: Action, status: Status) -> Self {
        Self { action, status }
    }

    pub async fn save<'a, X>(&self, conn: X) -> sqlx::Result<u64>
    where
        X: sqlx::Executor<'a, Database = sqlx::Sqlite>,
    {
        let sql = "INSERT INTO tasks (action, status) VALUES (?, ?) RETURNING id";
        sqlx::query_scalar(sql)
            .persistent(true)
            .bind(Json(&self.action))
            .bind(Json(&self.status))
            .fetch_one(conn)
            .await
    }
}

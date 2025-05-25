use chrono::{DateTime, Utc};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Task {
    pub id: u64,
    pub description: String,
    pub status: TaskStatus,
    pub retry: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum TaskStatus {
    Waiting,
    Progress,
    Canceled,
    Completed,
    Failed,
}

#[derive(Debug)]
pub struct Entity {
    pub file_id: u64,
    pub episode_id: u64,
}

impl Entity {
    pub fn new(file_id: u64, episode_id: u64) -> Self {
        Self {
            file_id,
            episode_id,
        }
    }

    pub async fn upsert<'a, X>(&self, conn: X) -> sqlx::Result<()>
    where
        X: sqlx::Executor<'a, Database = sqlx::Sqlite>,
    {
        sqlx::query(r#"INSERT INTO tvshow_episode_files (file_id, episode_id) VALUES (?,?) ON CONFLICT DO NOTHING"#)
            .bind(self.file_id as i64)
            .bind(self.episode_id as i64)
            .execute(conn).await?;
        Ok(())
    }
}

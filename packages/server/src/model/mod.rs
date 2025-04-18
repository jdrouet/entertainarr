pub(crate) mod tvshow;
pub(crate) mod user;

#[derive(Clone, Debug, Default, serde::Deserialize)]
pub struct Dataset {
    #[serde(default)]
    pub users: Vec<user::Entity>,
}

impl Dataset {
    pub async fn preload<'a>(&self, database: &sqlx::SqlitePool) -> sqlx::Result<()> {
        let mut tx = database.begin().await?;
        for user in self.users.iter() {
            user.persist(&mut *tx).await?;
        }
        tx.commit().await
    }
}

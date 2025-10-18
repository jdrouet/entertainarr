use anyhow::Context;

use entertainarr_domain::auth::entity::Profile;
use entertainarr_domain::auth::prelude::SignupError;

const FIND_BY_CREDS_QUERY: &str = "select id from users where email = ? limit 1";
const CREATE_QUERY: &str = "insert into users (email, password) values (?, ?) returning id";

impl entertainarr_domain::auth::prelude::AuthenticationRepository for super::Pool {
    #[tracing::instrument(
        skip_all,
        fields(
            otel.kind = "client",
            db.system = "sqlite",
            db.name = "authentication",
            db.operation = "SELECT",
            db.sql.table = "users",
            db.query.text = FIND_BY_CREDS_QUERY,
            db.response.returned_rows = tracing::field::Empty,
            error.type = tracing::field::Empty,
            error.message = tracing::field::Empty,
            error.stacktrace = tracing::field::Empty,
        ),
        err(Debug),
    )]
    async fn find_by_credentials(
        &self,
        email: &str,
        password: &str,
    ) -> anyhow::Result<Option<Profile>> {
        sqlx::query_as(FIND_BY_CREDS_QUERY)
            .bind(email)
            .bind(password)
            .fetch_optional(&self.0)
            .await
            .inspect(super::record_optional)
            .inspect_err(super::record_error)
            .map(super::Wrapper::maybe_inner)
            .context("unable to fetch profile by credentials")
    }

    #[tracing::instrument(
        skip_all,
        fields(
            otel.kind = "client",
            db.system = "sqlite",
            db.name = "authentication",
            db.operation = "insert",
            db.sql.table = "users",
            db.query.text = CREATE_QUERY,
            db.response.returned_rows = tracing::field::Empty,
            error.type = tracing::field::Empty,
            error.message = tracing::field::Empty,
            error.stacktrace = tracing::field::Empty,
        ),
        err(Debug),
    )]
    async fn create(&self, email: &str, password: &str) -> Result<Profile, SignupError> {
        sqlx::query_as(CREATE_QUERY)
            .bind(email)
            .bind(password)
            .fetch_one(&self.0)
            .await
            .inspect(super::record_one)
            .inspect_err(super::record_error)
            .map(super::Wrapper::inner)
            .map_err(|err| match err.as_database_error() {
                Some(dberr) if dberr.is_unique_violation() => SignupError::EmailConflict,
                _ => SignupError::Internal(anyhow::Error::from(err)),
            })
    }
}

impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow>
    for super::Wrapper<entertainarr_domain::auth::entity::Profile>
{
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        use entertainarr_domain::auth::entity::Profile;
        use sqlx::Row;

        Ok(Self(Profile {
            id: row.try_get(0)?,
        }))
    }
}

#[cfg(test)]
mod tests {
    use entertainarr_domain::auth::prelude::{AuthenticationRepository, SignupError};

    #[tokio::test]
    async fn should_not_find_user_by_creds_when_missing() {
        let tmpdir = tempfile::tempdir().unwrap();
        let pool = crate::Pool::test(&tmpdir.path().join("db")).await;
        let res = pool
            .find_by_credentials("user@example.com", "password")
            .await
            .unwrap();
        assert!(res.is_none());
    }

    #[tokio::test]
    async fn should_find_user_by_creds_when_exists() {
        let tmpdir = tempfile::tempdir().unwrap();
        let pool = crate::Pool::test(&tmpdir.path().join("db")).await;
        sqlx::query("insert into users (email, password) values ('user@example.com', 'password')")
            .execute(&pool.0)
            .await
            .unwrap();
        let res = pool
            .find_by_credentials("user@example.com", "password")
            .await
            .unwrap();
        assert!(res.is_some());
    }

    #[tokio::test]
    async fn should_create() {
        let tmpdir = tempfile::tempdir().unwrap();
        let pool = crate::Pool::test(&tmpdir.path().join("db")).await;
        let res = pool.create("user@example.com", "password").await.unwrap();
        assert_eq!(res.id, 1);
    }

    #[tokio::test]
    async fn should_not_create_if_exists() {
        let tmpdir = tempfile::tempdir().unwrap();
        let pool = crate::Pool::test(&tmpdir.path().join("db")).await;
        let res = pool.create("user@example.com", "password").await.unwrap();
        assert_eq!(res.id, 1);
        let err = pool
            .create("user@example.com", "password")
            .await
            .unwrap_err();
        assert!(matches!(err, SignupError::EmailConflict));
    }
}

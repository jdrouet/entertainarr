use anyhow::Context;

use crate::domain::auth::entity::Profile;
use crate::domain::auth::prelude::SignupError;

const FIND_BY_CREDS_QUERY: &str = "select id from users where email = ? limit 1";
const CREATE_QUERY: &str = "insert into users (email, password) values (?, ?) returning id";

impl crate::domain::auth::prelude::AuthenticationRepository for super::Pool {
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
            .inspect(|row| {
                let span = tracing::Span::current();
                span.record(
                    "db.response.returned_rows",
                    if row.is_some() { 1 } else { 0 },
                );
            })
            .inspect_err(|err| {
                let span = tracing::Span::current();
                span.record(
                    "error.type",
                    if err.as_database_error().is_some() {
                        "client"
                    } else {
                        "server"
                    },
                );
                span.record("error.message", err.to_string());
                span.record("error.stacktrace", format!("{err:?}"));
            })
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
            .inspect(|_| {
                let span = tracing::Span::current();
                span.record("db.response.returned_rows", 1);
            })
            .inspect_err(|err| {
                let span = tracing::Span::current();
                span.record(
                    "error.type",
                    if err.as_database_error().is_some() {
                        "client"
                    } else {
                        "server"
                    },
                );
                span.record("error.message", err.to_string());
                span.record("error.stacktrace", format!("{err:?}"));
            })
            .map(super::Wrapper::inner)
            .map_err(|err| match err.as_database_error() {
                Some(dberr) if dberr.is_unique_violation() => SignupError::EmailConflict,
                _ => SignupError::Internal(anyhow::Error::from(err)),
            })
    }
}

impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow>
    for super::Wrapper<crate::domain::auth::entity::Profile>
{
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        use crate::domain::auth::entity::Profile;
        use sqlx::Row;

        Ok(Self(Profile {
            id: row.try_get(0)?,
        }))
    }
}

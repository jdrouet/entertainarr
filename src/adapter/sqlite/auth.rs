use anyhow::Context;

const FIND_BY_EMAIL_QUERY: &str = "select id, password from users where email = ? limit 1";

impl crate::domain::auth::prelude::AuthenticationRepository for super::Pool {
    #[tracing::instrument(
        skip_all,
        fields(
            otel.kind = "client",
            db.system = "sqlite",
            db.name = "authentication",
            db.operation = "SELECT",
            db.sql.table = "users",
            db.query.text = FIND_BY_EMAIL_QUERY,
            db.response.returned_rows = tracing::field::Empty,
            error.type = tracing::field::Empty,
            error.message = tracing::field::Empty,
            error.stacktrace = tracing::field::Empty,
        ),
        err(Debug),
    )]
    async fn find_by_email(
        &self,
        email: &str,
    ) -> anyhow::Result<Option<crate::domain::auth::entity::Profile>> {
        sqlx::query_as(FIND_BY_EMAIL_QUERY)
            .bind(email)
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
            .context("unable to fetch profile by email")
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
            password: row.try_get(1)?,
        }))
    }
}

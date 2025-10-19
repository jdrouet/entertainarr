use anyhow::Context;

use crate::Wrapper;
use entertainarr_domain::podcast::entity::PodcastEpisode;
use entertainarr_domain::podcast::prelude::{ListPodcastEpisodeParams, PodcastEpisodeField};
use entertainarr_domain::prelude::SortOrder;

impl entertainarr_domain::podcast::prelude::PodcastEpisodeRepository for super::Pool {
    #[tracing::instrument(
        skip_all,
        fields(
            otel.kind = "client",
            db.system = "sqlite",
            db.name = "podcast",
            db.operation = "SELECT",
            db.sql.table = "podcast_episodes",
            db.query.text = tracing::field::Empty,
            db.response.returned_rows = tracing::field::Empty,
            error.type = tracing::field::Empty,
            error.message = tracing::field::Empty,
            error.stacktrace = tracing::field::Empty,
        ),
        err(Debug),
    )]
    async fn list(&self, params: ListPodcastEpisodeParams) -> anyhow::Result<Vec<PodcastEpisode>> {
        let mut qb: sqlx::QueryBuilder<'_, sqlx::Sqlite> = sqlx::QueryBuilder::new(
            r#"select
    podcast_episodes.id,
    podcast_episodes.podcast_id,
    podcast_episodes.guid,
    podcast_episodes.published_at,
    podcast_episodes.title,
    podcast_episodes.description,
    podcast_episodes.link,
    podcast_episodes.duration,
    podcast_episodes.file_url,
    podcast_episodes.file_size,
    podcast_episodes.file_type,
    podcast_episodes.created_at,
    podcast_episodes.updated_at
from podcast_episodes"#,
        );

        if let Some(subscribed) = params.filter.subscribed {
            if subscribed {
                qb.push(" join user_podcasts on user_podcasts.podcast_id = podcast_episodes.podcast_id and user_podcasts.user_id = ").push_bind(params.user_id as i64);
            } else {
                // TODO handle filtered those where the user is not subscribed
            }
        } else {
            // nothing to do here
        }

        if let Some(watched) = params.filter.watched {
            qb.push(" left outer join user_podcast_episodes on user_podcast_episodes.podcast_episode_id = podcast_episodes.id ");
            qb.push(" and user_podcast_episodes.user_id = ")
                .push_bind(params.user_id as i64);
            if watched {
                qb.push(" where user_podcast_episodes.completed");
            } else {
                qb.push(" where (user_podcast_episodes.completed is null or not user_podcast_episodes.completed)");
            }
        }

        match params.sort.field {
            PodcastEpisodeField::PublishedAt => qb.push(" order by podcast_episodes.published_at"),
        };
        match params.sort.order {
            SortOrder::Asc => qb.push(" asc"),
            SortOrder::Desc => qb.push(" desc"),
        };

        qb.push(" limit ")
            .push_bind(params.page.limit)
            .push(" offset ")
            .push_bind(params.page.offset);

        let span = tracing::Span::current();
        span.record("db.query.text", qb.sql());

        qb.build_query_as()
            .fetch_all(&self.0)
            .await
            .inspect(super::record_all)
            .inspect_err(super::record_error)
            .map(Wrapper::list)
            .context("unable to query podcast episodes")
    }
}

impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for super::Wrapper<PodcastEpisode> {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        use sqlx::Row;

        Ok(Self(PodcastEpisode {
            id: row.try_get(0)?,
            podcast_id: row.try_get(1)?,
            guid: row.try_get(2)?,
            published_at: row.try_get(3)?,
            title: row.try_get(4)?,
            description: row.try_get(5)?,
            link: row.try_get(6)?,
            duration: row
                .try_get(7)
                .map(|value: Option<u64>| value.map(std::time::Duration::from_secs))?,
            file_url: row.try_get(8)?,
            file_size: row.try_get(9)?,
            file_type: row.try_get(10)?,
            created_at: row.try_get(11)?,
            updated_at: row.try_get(12)?,
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::Pool;
    use entertainarr_domain::podcast::prelude::{
        ListPodcastEpisodeFilter, ListPodcastEpisodeParams, PodcastEpisodeField,
        PodcastEpisodeRepository,
    };
    use entertainarr_domain::prelude::{Page, Sort, SortOrder};

    async fn seed(pool: &Pool) {
        let _: Vec<u64> = sqlx::query_scalar("insert into users (id, email, password) values (1, 'user1@example.com', 'password'), (2, 'user2@example.com', 'password') returning id").fetch_all(pool.as_ref()).await.unwrap();
        let _: Vec<u64> = sqlx::query_scalar("insert into podcasts (id, feed_url, title) values (1, 'first', 'first'), (2, 'second', 'second'), (3, 'third', 'third') returning id").fetch_all(pool.as_ref()).await.unwrap();
        let _: Vec<u64> = sqlx::query_scalar("insert into podcast_episodes (id, podcast_id, title, file_url) values (1, 1, 'title 1', 'url 1'), (2, 1, 'title 2', 'url 2'), (3, 1, 'title 3', 'url 3'), (4, 2, 'title 4', 'url 4'), (5, 2, 'title 5', 'url 5'), (6, 3, 'title 6', 'url 6') returning id").fetch_all(pool.as_ref()).await.unwrap();
        let _: Vec<(u64, u64)> = sqlx::query_as(
            "insert into user_podcasts (user_id, podcast_id) values (1, 1), (1, 2), (2, 2), (2, 3) returning user_id, podcast_id",
        )
        .fetch_all(pool.as_ref())
        .await
        .unwrap();
        let _: Vec<(u64, u64)> = sqlx::query_as("insert into user_podcast_episodes (user_id, podcast_episode_id, progress, completed) values (1, 1, 123, true), (1, 4, 10, false), (2, 4, 153, true), (2, 5, 10, false) returning user_id, podcast_episode_id").fetch_all(pool.as_ref()).await.unwrap();
    }

    #[tokio::test]
    async fn should_list_all_subscribed_episodes() {
        let _ = tracing_subscriber::fmt::try_init();

        let tmpdir = tempfile::tempdir().unwrap();
        let pool = crate::Pool::test(&tmpdir.path().join("db")).await;

        seed(&pool).await;
        let list = pool
            .list(ListPodcastEpisodeParams {
                user_id: 1,
                filter: ListPodcastEpisodeFilter {
                    subscribed: Some(true),
                    watched: None,
                },
                sort: Sort {
                    field: PodcastEpisodeField::PublishedAt,
                    order: SortOrder::Asc,
                },
                page: Page {
                    limit: 10,
                    offset: 0,
                },
            })
            .await
            .unwrap();
        assert_eq!(list.len(), 5);
    }

    #[tokio::test]
    async fn should_list_all_watched_episodes() {
        let _ = tracing_subscriber::fmt::try_init();

        let tmpdir = tempfile::tempdir().unwrap();
        let pool = crate::Pool::test(&tmpdir.path().join("db")).await;

        seed(&pool).await;
        let list = pool
            .list(ListPodcastEpisodeParams {
                user_id: 1,
                filter: ListPodcastEpisodeFilter {
                    subscribed: Some(true),
                    watched: Some(true),
                },
                sort: Sort {
                    field: PodcastEpisodeField::PublishedAt,
                    order: SortOrder::Asc,
                },
                page: Page {
                    limit: 10,
                    offset: 0,
                },
            })
            .await
            .unwrap();
        assert_eq!(list.len(), 1);
    }

    #[tokio::test]
    async fn should_list_all_unwatched_episodes() {
        let _ = tracing_subscriber::fmt::try_init();

        let tmpdir = tempfile::tempdir().unwrap();
        let pool = crate::Pool::test(&tmpdir.path().join("db")).await;

        seed(&pool).await;
        let list = pool
            .list(ListPodcastEpisodeParams {
                user_id: 1,
                filter: ListPodcastEpisodeFilter {
                    subscribed: Some(true),
                    watched: Some(false),
                },
                sort: Sort {
                    field: PodcastEpisodeField::PublishedAt,
                    order: SortOrder::Asc,
                },
                page: Page {
                    limit: 10,
                    offset: 0,
                },
            })
            .await
            .unwrap();
        assert_eq!(list.len(), 4);
    }
}

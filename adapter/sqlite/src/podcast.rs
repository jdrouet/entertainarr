use std::collections::HashSet;

use anyhow::Context;
use tracing::Instrument;

use crate::Wrapper;
use entertainarr_domain::podcast::entity::{Podcast, PodcastInput};

const FIND_PODCAST_BY_FEED_URL_QUERY: &str = "select id, feed_url, title, description, image_url, language, website, created_at, updated_at from podcasts where feed_url like ? limit 1";
const UPSERT_PODCAST_QUERY: &str = r#"insert into podcasts (feed_url, title, description, image_url, language, website)
values (?, ?, ?, ?, ?, ?)
on conflict (feed_url) do update set
    title=excluded.title,
    description=excluded.description,
    image_url=excluded.image_url,
    language=excluded.language,
    website=excluded.website,
    updated_at=CURRENT_TIMESTAMP
returning id, feed_url, title, description, image_url, language, website, created_at, updated_at"#;
const LIST_PODCAST_QUERY: &str = r#"select podcasts.id, podcasts.feed_url, podcasts.title, podcasts.description, podcasts.image_url, podcasts.language, podcasts.website, podcasts.created_at, podcasts.updated_at
from podcasts
join user_podcasts on podcasts.id = user_podcasts.podcast_id
where user_podcasts.user_id = ?
order by podcasts.title"#;
const UPSERT_USER_PODCAST_QUERY: &str = "insert into user_podcasts (user_id, podcast_id) values (?, ?) on conflict (user_id, podcast_id) do nothing";
const DELETE_USER_PODCAST_QUERY: &str =
    "delete from user_podcasts where user_id = ? and podcast_id = ?";

impl entertainarr_domain::podcast::prelude::PodcastRepository for super::Pool {
    #[tracing::instrument(
        skip_all,
        fields(
            otel.kind = "client",
            db.system = "sqlite",
            db.name = "podcast",
            db.operation = "SELECT",
            db.sql.table = "podcasts",
            db.query.text = FIND_PODCAST_BY_FEED_URL_QUERY,
            db.response.returned_rows = tracing::field::Empty,
            error.type = tracing::field::Empty,
            error.message = tracing::field::Empty,
            error.stacktrace = tracing::field::Empty,
        ),
        err(Debug),
    )]
    async fn find_by_feed_url(&self, feed_url: &str) -> anyhow::Result<Option<Podcast>> {
        sqlx::query_as(FIND_PODCAST_BY_FEED_URL_QUERY)
            .bind(feed_url)
            .fetch_optional(&self.0)
            .await
            .inspect(super::record_optional)
            .inspect_err(super::record_error)
            .map(Wrapper::maybe_inner)
            .context("unable to query podcasts by feed url")
    }

    #[tracing::instrument(
        skip_all,
        fields(
            otel.kind = "client",
            db.system = "sqlite",
            db.name = "podcast",
            db.operation = "SELECT",
            db.sql.table = "podcasts",
            db.query.text = tracing::field::Empty,
            db.response.returned_rows = tracing::field::Empty,
            error.type = tracing::field::Empty,
            error.message = tracing::field::Empty,
            error.stacktrace = tracing::field::Empty,
        ),
        err(Debug),
    )]
    async fn list_by_ids(&self, podcast_ids: &[u64]) -> anyhow::Result<Vec<Podcast>> {
        if podcast_ids.is_empty() {
            return Ok(Vec::default());
        }
        let mut qb = sqlx::QueryBuilder::new(
            "select podcasts.id, podcasts.feed_url, podcasts.title, podcasts.description, podcasts.image_url, podcasts.language, podcasts.website, podcasts.created_at, podcasts.updated_at from podcasts where",
        );
        let mut known = HashSet::with_capacity(podcast_ids.len());
        for id in podcast_ids {
            if !known.contains(id) {
                if !known.is_empty() {
                    qb.push(" and");
                }
                qb.push(" id = ").push_bind(*id as i64);
                known.insert(id);
            }
        }
        qb.build_query_as()
            .fetch_all(&self.0)
            .await
            .inspect(super::record_all)
            .inspect_err(super::record_error)
            .map(Wrapper::list)
            .context("unable to query podcasts by feed url")
    }

    async fn upsert(&self, entity: &PodcastInput) -> anyhow::Result<Podcast> {
        let mut tx = self
            .0
            .begin()
            .await
            .context("unable to begin transaction")?;

        let span = tracing::info_span!(
            "podcasts.upsert",
            otel.kind = "client",
            db.system = "sqlite",
            db.name = "podcast",
            db.operation = "UPSERT",
            db.sql.table = "podcasts",
            db.query.text = UPSERT_PODCAST_QUERY,
            db.response.returned_rows = tracing::field::Empty,
            error.type = tracing::field::Empty,
            error.message = tracing::field::Empty,
            error.stacktrace = tracing::field::Empty,
        );
        let podcast: Podcast = sqlx::query_as(UPSERT_PODCAST_QUERY)
            .bind(&entity.feed_url)
            .bind(&entity.title)
            .bind(entity.description.as_ref())
            .bind(entity.image_url.as_ref())
            .bind(entity.language.as_ref())
            .bind(entity.website.as_ref())
            .fetch_one(&mut *tx)
            .instrument(span)
            .await
            .inspect(super::record_one)
            .inspect_err(super::record_error)
            .map(Wrapper::inner)
            .context("unable to upsert podcast")?;

        let mut qb: sqlx::QueryBuilder<'_, sqlx::Sqlite> = sqlx::QueryBuilder::new(
            "insert into podcast_episodes (podcast_id, guid, published_at, title, description, link, duration, file_url, file_size, file_type)",
        );
        qb.push_values(entity.episodes.iter(), |mut b, item| {
            b.push_bind(podcast.id as i64)
                .push_bind(&item.guid)
                .push_bind(item.published_at)
                .push_bind(&item.title)
                .push_bind(&item.description)
                .push_bind(&item.link)
                .push_bind(item.duration.as_ref().map(|value| value.as_secs() as i64))
                .push_bind(&item.file_url)
                .push_bind(item.file_size.map(|value| value as i64))
                .push_bind(&item.file_type);
        });
        qb.push(" on conflict (podcast_id, guid) do nothing");

        let span = tracing::info_span!(
            "podcast_episodes.upsert",
            otel.kind = "client",
            db.system = "sqlite",
            db.name = "podcast",
            db.operation = "UPSERT",
            db.sql.table = "podcast_episodes",
            db.query.text = qb.sql(),
            db.response.returned_rows = tracing::field::Empty,
            error.type = tracing::field::Empty,
            error.message = tracing::field::Empty,
            error.stacktrace = tracing::field::Empty,
        );
        qb.build()
            .execute(&mut *tx)
            .instrument(span)
            .await
            .inspect_err(super::record_error)?;

        tx.commit().await.context("unable to commit transaction")?;
        Ok(podcast)
    }
}

impl entertainarr_domain::podcast::prelude::PodcastSubscriptionRepository for super::Pool {
    #[tracing::instrument(
        skip_all,
        fields(
            otel.kind = "client",
            db.system = "sqlite",
            db.name = "podcast",
            db.operation = "UPSERT",
            db.sql.table = "user_podcasts",
            db.query.text = UPSERT_USER_PODCAST_QUERY,
            db.response.returned_rows = tracing::field::Empty,
            error.type = tracing::field::Empty,
            error.message = tracing::field::Empty,
            error.stacktrace = tracing::field::Empty,
        ),
        err(Debug),
    )]
    async fn create(&self, user_id: u64, podcast_id: u64) -> anyhow::Result<()> {
        sqlx::query(UPSERT_USER_PODCAST_QUERY)
            .bind(user_id as i64)
            .bind(podcast_id as i64)
            .execute(&self.0)
            .await
            .inspect_err(super::record_error)
            .map(|_| ())
            .context("unable to upsert user podcast relation")
    }

    #[tracing::instrument(
        skip_all,
        fields(
            otel.kind = "client",
            db.system = "sqlite",
            db.name = "podcast",
            db.operation = "DELETE",
            db.sql.table = "user_podcasts",
            db.query.text = DELETE_USER_PODCAST_QUERY,
            db.response.returned_rows = tracing::field::Empty,
            error.type = tracing::field::Empty,
            error.message = tracing::field::Empty,
            error.stacktrace = tracing::field::Empty,
        ),
        err(Debug),
    )]
    async fn delete(&self, user_id: u64, podcast_id: u64) -> anyhow::Result<()> {
        sqlx::query(DELETE_USER_PODCAST_QUERY)
            .bind(user_id as i64)
            .bind(podcast_id as i64)
            .execute(&self.0)
            .await
            .inspect_err(super::record_error)
            .map(|_| ())
            .context("unable to delete user podcast relation")
    }

    #[tracing::instrument(
        skip_all,
        fields(
            otel.kind = "client",
            db.system = "sqlite",
            db.name = "podcast",
            db.operation = "SELECT",
            db.sql.table = "podcasts",
            db.query.text = LIST_PODCAST_QUERY,
            db.response.returned_rows = tracing::field::Empty,
            error.type = tracing::field::Empty,
            error.message = tracing::field::Empty,
            error.stacktrace = tracing::field::Empty,
        ),
        err(Debug),
    )]
    async fn list(
        &self,
        user_id: u64,
    ) -> anyhow::Result<Vec<entertainarr_domain::podcast::entity::Podcast>> {
        sqlx::query_as(LIST_PODCAST_QUERY)
            .bind(user_id as i64)
            .fetch_all(&self.0)
            .await
            .inspect(super::record_all)
            .inspect_err(super::record_error)
            .map(Wrapper::list)
            .context("unable to list podcasts")
    }
}

impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for super::Wrapper<Podcast> {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        use sqlx::Row;

        Ok(Self(Podcast {
            id: row.try_get(0)?,
            feed_url: row.try_get(1)?,
            title: row.try_get(2)?,
            description: row.try_get(3)?,
            image_url: row.try_get(4)?,
            language: row.try_get(5)?,
            website: row.try_get(6)?,
            created_at: row.try_get(7)?,
            updated_at: row.try_get(8)?,
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::Pool;
    use entertainarr_domain::podcast::{
        entity::{PodcastEpisodeInput, PodcastInput},
        prelude::PodcastRepository,
    };

    #[tokio::test]
    async fn should_find_episode_by_feed_url_when_existing() {
        let tmpdir = tempfile::tempdir().unwrap();
        let pool = crate::Pool::test(&tmpdir.path().join("db")).await;

        let id: u64 = sqlx::query_scalar(
            "insert into podcasts (feed_url, title) values ('http://foo.bar/atom.rss', 'Foo Bar') returning id",
        )
        .fetch_one(&pool.0)
        .await
        .unwrap();

        let res = pool
            .find_by_feed_url("http://foo.bar/atom.rss")
            .await
            .unwrap()
            .unwrap();

        assert_eq!(id, res.id);
    }

    #[tokio::test]
    async fn should_find_episode_by_feed_url_when_missing() {
        let tmpdir = tempfile::tempdir().unwrap();
        let pool = crate::Pool::test(&tmpdir.path().join("db")).await;
        let res = pool.find_by_feed_url("missing_url").await.unwrap();
        assert!(res.is_none());
    }

    async fn count_postcasts(pool: &Pool) -> u32 {
        sqlx::query_scalar("select count(*) from podcasts")
            .fetch_one(&pool.0)
            .await
            .unwrap()
    }

    async fn count_postcast_episodes(pool: &Pool) -> u32 {
        sqlx::query_scalar("select count(*) from podcast_episodes")
            .fetch_one(&pool.0)
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn should_upsert_podcast_and_episodes() {
        let tmpdir = tempfile::tempdir().unwrap();
        let pool = crate::Pool::test(&tmpdir.path().join("db")).await;

        let content = PodcastInput {
            feed_url: "http://example.com/atom.rss".into(),
            title: "Example".into(),
            description: None,
            image_url: None,
            language: None,
            website: None,
            episodes: vec![
                PodcastEpisodeInput {
                    guid: Some("aaaaa".into()),
                    published_at: None,
                    title: "First episode".into(),
                    description: None,
                    link: None,
                    duration: None,
                    file_url: "http://example.com/first.mp3".into(),
                    file_size: None,
                    file_type: None,
                },
                PodcastEpisodeInput {
                    guid: Some("aaaab".into()),
                    published_at: None,
                    title: "Second episode".into(),
                    description: None,
                    link: None,
                    duration: None,
                    file_url: "http://example.com/second.mp3".into(),
                    file_size: None,
                    file_type: None,
                },
            ],
        };
        let _res = pool.upsert(&content).await.unwrap();
        assert_eq!(count_postcasts(&pool).await, 1);
        assert_eq!(count_postcast_episodes(&pool).await, 2);
        // should not recreate it
        let _res = pool.upsert(&content).await.unwrap();
        assert_eq!(count_postcasts(&pool).await, 1);
        assert_eq!(count_postcast_episodes(&pool).await, 2);
    }
}

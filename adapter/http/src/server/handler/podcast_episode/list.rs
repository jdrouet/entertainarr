use crate::entity::ApiResource;
use crate::entity::podcast::PodcastDocument;
use crate::entity::podcast_episode::{PodcastEpisodeDocument, PodcastEpisodeRelation};
use axum::Json;
use axum::extract::State;
use serde_qs::axum::QsQuery;

use crate::server::extractor::user::CurrentUser;
use crate::server::handler::ApiError;
use crate::server::handler::podcast_episode::PodcastEpisodeField;
use crate::server::handler::prelude::{Page, Sort};
use entertainarr_domain::podcast::prelude::{
    ListPodcastEpisodeFilter, ListPodcastEpisodeParams, PodcastEpisodeService, PodcastService,
};

#[derive(Default, serde::Deserialize)]
pub struct QueryFilter {
    #[serde(default)]
    subscribed: Option<bool>,
    #[serde(default)]
    watched: Option<bool>,
}

impl From<QueryFilter> for ListPodcastEpisodeFilter {
    fn from(value: QueryFilter) -> Self {
        Self {
            subscribed: value.subscribed,
            watched: value.watched,
        }
    }
}

#[derive(serde::Deserialize)]
pub struct QueryParams {
    #[serde(default)]
    filter: QueryFilter,
    #[serde(default)]
    sort: Sort<PodcastEpisodeField>,
    #[serde(default)]
    page: Page,
}

pub async fn handle<S>(
    State(state): State<S>,
    CurrentUser(user_id): CurrentUser,
    QsQuery(params): QsQuery<QueryParams>,
) -> Result<Json<ApiResource<Vec<PodcastEpisodeDocument>, PodcastEpisodeRelation>>, ApiError>
where
    S: crate::server::prelude::ServerState,
{
    let list = state
        .podcast_episode_service()
        .list(ListPodcastEpisodeParams {
            user_id,
            filter: params.filter.into(),
            sort: params.sort.into(),
            page: params.page.into(),
        })
        .await
        .map_err(|err| {
            tracing::error!(error = ?err, "unable to list podcast episodes");
            ApiError::internal()
        })?;
    let podcast_ids = list.iter().map(|p| p.podcast_id).collect::<Vec<_>>();
    let podcasts = state
        .podcast_service()
        .list_by_ids(&podcast_ids)
        .await
        .map_err(|err| {
            tracing::error!(error = ?err, "unable to list podcasts");
            ApiError::internal()
        })?;

    let data = list
        .into_iter()
        .map(super::PodcastEpisodeDocument::from)
        .collect::<Vec<_>>();

    let includes = podcasts
        .into_iter()
        .map(PodcastDocument::from)
        .map(PodcastEpisodeRelation::Podcast)
        .collect::<Vec<_>>();

    Ok(Json(ApiResource { data, includes }))
}

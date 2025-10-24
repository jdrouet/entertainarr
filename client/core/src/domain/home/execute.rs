use crux_http::command::Http;
use entertainarr_adapter_http::entity::{
    ApiResource,
    podcast_episode::{PodcastEpisodeDocument, PodcastEpisodeRelation},
};

use crate::entity::podcast_episode::PodcastEpisode;

pub fn list_podcast_episodes(
    base_url: &str,
    token: &str,
) -> crux_core::Command<crate::Effect, crate::Event> {
    let url = format!("{base_url}/api/podcast-episodes");
    Http::get(url)
        .header("authorization", format!("Bearer {token}"))
        .expect_json::<ApiResource<Vec<PodcastEpisodeDocument>, PodcastEpisodeRelation>>()
        .build()
        .then_send(|res| {
            match res {
                Ok(mut res) => {
                    let payload = res.take_body().unwrap();
                    super::HomeEvent::ListPodcastEpisodesSuccess(
                        PodcastEpisode::from_episode_document_list(payload),
                    )
                }
                Err(err) => super::HomeEvent::ListPodcastEpisodesError(err),
            }
            .into()
        })
}

use crux_http::command::Http;
use entertainarr_adapter_http::entity::{
    ApiResource,
    podcast_episode::{PodcastEpisodeDocument, PodcastEpisodeRelation},
};

use crate::entity::podcast_episode::PodcastEpisode;

pub fn list_podcast_episodes(base_url: &str, token: &str) -> crate::ApplicationCommand {
    let url = format!("{base_url}/api/podcast-episodes");
    Http::get(url)
        .query(&serde_json::json!({
            "include": "podcast",
            "sort": "-published_at",
        }))
        .unwrap()
        .header("Authorization", format!("Bearer {token}"))
        .expect_json::<ApiResource<Vec<PodcastEpisodeDocument>, PodcastEpisodeRelation>>()
        .build()
        .then_send(|res| {
            match res {
                Ok(mut res) => {
                    let payload = res.take_body().unwrap();
                    let episodes = PodcastEpisode::from_episode_document_list(payload);
                    super::HomeEvent::ListPodcastEpisodesSuccess(episodes)
                }
                Err(_err) => super::HomeEvent::ListPodcastEpisodesError,
            }
            .into()
        })
}

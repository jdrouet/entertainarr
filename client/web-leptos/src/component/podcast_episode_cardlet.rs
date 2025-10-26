use entertainarr_client_core::entity::podcast_episode::PodcastEpisode;
use leptos::prelude::*;

stylance::import_style!(style, "podcast_episode_cardlet.module.scss");

#[component]
pub fn PodcastEpisodeCardlet(episode: PodcastEpisode) -> impl IntoView {
    view! {
        <div class=style::podcast_episode_cardlet>
            <img class=style::episode_image src={episode.podcast_image_url.clone()} alt={episode.podcast_title.clone()} />
            <div class=style::episode_details>
                <h3 class=style::episode_title>{episode.title}</h3>
                <p class=style::episode_date>{episode.published_at.unwrap_or_default()}</p>
            </div>
        </div>
    }
}

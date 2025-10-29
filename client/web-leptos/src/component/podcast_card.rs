use entertainarr_client_core::entity::podcast::Podcast;
use leptos::prelude::*;

stylance::import_style!(style, "podcast_card.module.scss");

#[component]
pub fn PodcastCard(podcast: Podcast) -> impl IntoView {
    view! {
        <div class={style::podcast_card}>
            <img src={podcast.image_url.clone()} alt={podcast.title.clone()} />
            <div class={style::podcast_card_content}>
                <h3>{podcast.title}</h3>
            </div>
        </div>
    }
}

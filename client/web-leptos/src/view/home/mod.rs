use entertainarr_client_core::Event;
use leptos::prelude::*;

use crate::context::core::use_events;

stylance::import_style!(style, "style.scss");

#[component]
pub fn View(model: entertainarr_client_core::domain::home::View) -> impl IntoView {
    let (_, on_change) = use_events();

    let on_subscribe = move |_| {
        on_change.set(Event::PodcastSubscription(
            entertainarr_client_core::domain::podcast_subscription::Event::Open,
        ));
    };

    view! {
        <crate::component::fullscreen_layout::FullscreenLayout classname={style::home}>
            <h1>{"Podcast Episodes"}</h1>

            {model.podcast_episodes.is_empty().then_some(view! {<div>{"Nothing to listen to..."}</div>})}

            <div class=style::episodes_grid>
                {model.podcast_episodes.into_iter().map(|episode| {
                    view! {
                        <crate::component::podcast_episode_cardlet::PodcastEpisodeCardlet episode />
                    }
                }).collect::<Vec<_>>()}
            </div>

            <button on:click=on_subscribe>{"Subscribe"}</button>
        </crate::component::fullscreen_layout::FullscreenLayout>
    }
}

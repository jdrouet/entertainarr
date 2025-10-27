use entertainarr_client_core::application::ApplicationEvent;
use entertainarr_client_core::application::authenticated::home::HomeModel;
use entertainarr_client_core::application::router::Route;
use leptos::prelude::*;
use web_sys::MouseEvent;

use crate::context::core::use_events;

stylance::import_style!(style, "home.module.scss");

#[component]
fn Section<F>(title: &'static str, on_subscribe: F, children: Children) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    view! {
        <section>
            <div class={style::section_header}>
                <h1>{title}</h1>

                <button on:click={on_subscribe}>{"Subscribe"}</button>
            </div>
            <div class={style::hscroll}>
                {children()}
            </div>
        </section>
    }
}

#[component]
pub fn View(model: HomeModel) -> impl IntoView {
    let (_, on_change) = use_events();

    let on_subscribe = move |_| {
        on_change.set(ApplicationEvent::RouteChange(Route::PodcastSubscribe));
    };

    view! {
        <crate::component::fullscreen::layout::FullscreenLayout>
            <Section title="Podcast Episodes" on_subscribe>
                {model.podcast_episodes.into_iter().map(|episode| {
                    view! {
                        <crate::component::podcast_episode_cardlet::PodcastEpisodeCardlet episode />
                    }
                }).collect::<Vec<_>>()}
            </Section>
        </crate::component::fullscreen::layout::FullscreenLayout>
    }
}

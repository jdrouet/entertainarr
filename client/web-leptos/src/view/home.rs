use leptos::prelude::*;

stylance::import_style!(style, "home.module.scss");

#[component]
pub fn HomeView(model: entertainarr_client_core::domain::home::HomeView) -> impl IntoView {
    view! {
        <crate::component::fullscreen_layout::FormLayout classname={style::home}>
            <h1>{"Podcast Episodes"}</h1>

            {model.podcast_episodes.is_empty().then_some(view! {<div>{"Nothing to listen to..."}</div>})}

            <div class=style::episodes_grid>
                {model.podcast_episodes.into_iter().map(|episode| {
                    view! {
                        <crate::component::podcast_episode_cardlet::PodcastEpisodeCardlet episode />
                    }
                }).collect::<Vec<_>>()}
            </div>
        </crate::component::fullscreen_layout::FormLayout>
    }
}

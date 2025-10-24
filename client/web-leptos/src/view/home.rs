use leptos::prelude::*;

stylance::import_style!(style, "home.module.scss");

#[component]
pub fn HomeView(model: entertainarr_client_core::domain::home::HomeView) -> impl IntoView {
    let (sidebar_opened, sidebar_toggle) = signal(false);

    let on_toggle_sidebar = move || sidebar_toggle.update(|prev| *prev = !*prev);

    view! {
        <div class={style::container}>
            <crate::component::header::Header on_toggle_sidebar />
            <crate::component::sidebar::Sidebar
                visible={sidebar_opened}
                on_close={move || sidebar_toggle.set(false)}
            />

            // Main content
            <main class=style::main_content>
                <h1>{"Podcast Episodes"}</h1>

                {model.podcast_episodes.is_empty().then_some(view! {<div>{"Nothing to listen to..."}</div>})}

                <div class=style::episodes_grid>
                    {model.podcast_episodes.into_iter().map(|episode| {
                        view! {
                            <crate::component::podcast_episode_cardlet::PodcastEpisodeCardlet episode />
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </main>
        </div>
    }
}

use leptos::prelude::*;

stylance::import_style!(style, "home.module.scss");

#[component]
pub fn HomeView(model: entertainarr_client_core::domain::home::HomeView) -> impl IntoView {
    let (sidebar_opened, sidebar_toggle) = signal(false);

    // TODO: Replace with actual podcast episode data
    let episodes = vec![
        Episode {
            id: 1,
            title: "Episode 1".to_string(),
            date: "2024-01-01".to_string(),
            image: "/placeholder-episode.jpg".to_string(),
        },
        Episode {
            id: 2,
            title: "Episode 2".to_string(),
            date: "2024-01-02".to_string(),
            image: "/placeholder-episode.jpg".to_string(),
        },
        Episode {
            id: 3,
            title: "Episode 3".to_string(),
            date: "2024-01-03".to_string(),
            image: "/placeholder-episode.jpg".to_string(),
        },
        Episode {
            id: 4,
            title: "Episode 4".to_string(),
            date: "2024-01-04".to_string(),
            image: "/placeholder-episode.jpg".to_string(),
        },
    ];

    let on_toggle_sidebar = move || sidebar_toggle.update(|prev| *prev = !*prev);

    view! {
        <div class={style::container}>
            <crate::component::header::Header on_toggle_sidebar />
            <crate::component::sidebar::Sidebar
                visible={sidebar_opened}
                on_close={move || sidebar_toggle.set(false)}
            />

            {if model.podcasts_loading { "Podcasts loading..."} else { "Podcasts not loading..."}}

            // Main content
            <main class=style::main_content>
                <h1>"Latest Unlistened Episodes"</h1>
                <div class=style::episodes_grid>
                    {episodes.into_iter().map(|episode| {
                        view! {
                            <div class=style::episode_card>
                                <img class=style::episode_image src=episode.image alt={episode.title.clone()} />
                                <div class=style::episode_details>
                                    <h3 class=style::episode_title>{episode.title}</h3>
                                    <p class=style::episode_date>{episode.date}</p>
                                </div>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </main>
        </div>
    }
}

#[derive(Clone)]
struct Episode {
    id: i32,
    title: String,
    date: String,
    image: String,
}

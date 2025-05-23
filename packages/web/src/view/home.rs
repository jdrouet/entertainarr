use yew::prelude::*;

use crate::component::empty_state::EmptyState;
use crate::component::error_message::ErrorMessage;
use crate::component::header::Header;
use crate::component::loading::Loading;
use crate::component::tvshow_episode_cardlet::TVShowEpisodeCardlet;
use crate::hook::tvshow_episode::use_episode_watchlist;

#[function_component(TVShowWatchlistSection)]
fn tvshow_watchlist_section() -> Html {
    let episode_watchlist = use_episode_watchlist();

    html! {
        <>
            <div class="flex flex-row justify-between items-center mb-4">
                <h1 class="text-2xl font-bold text-gray-800">
                    {"TV Show - Next episodes"}
                </h1>
            </div>
            if episode_watchlist.loading {
                <Loading classes="flex-col min-h-[200px]" />
            } else if episode_watchlist.error.is_some() {
                <ErrorMessage classes="min-h-[200px]" message={format!("Couldn't load watchlist...")} />
            } else if let Some(episodes) = &episode_watchlist.data {
                if episodes.is_empty() {
                    <EmptyState classes="min-h-[200px]" title="You're up to date!" subtitle="Add more TV shows for more entertainment..." />
                } else {
                    <div class="grid grid-cols-3 gap-4">
                        { for episodes.iter().map(|episode| html! {
                            <TVShowEpisodeCardlet episode={episode.clone()} />
                        }) }
                    </div>
                }
            } else {
                <EmptyState classes="min-h-[200px]" title="You're up to date!" subtitle="Add more TV shows for more entertainment..." />
            }
        </>
    }
}

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="bg-gray-100 min-h-screen">
            <Header />
            <main class="max-w-6xl mx-auto p-6">
                <TVShowWatchlistSection />
            </main>
        </div>
    }
}

use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::component::empty_state::EmptyState;
use crate::component::error_message::ErrorMessage;
use crate::component::header::Header;
use crate::component::loading::Loading;
use crate::component::tvshow_cardlet::TVShowCardlet;
use crate::hook::tvshow::use_followed_tvshows;

#[function_component(TVShowIndex)]
pub fn tvshow_index() -> Html {
    let tvshows = use_followed_tvshows();

    html! {
        <div class="bg-gray-100 min-h-screen">
            <Header />
            <main class="max-w-6xl mx-auto p-6">
                <div class="flex flex-row justify-between items-center mb-4">
                    <h1 class="text-2xl font-bold text-gray-800">
                        {"Followed TV Shows"}
                    </h1>
                    <Link<Route> to={Route::TvshowSearch} classes="text-sm px-4 py-2 rounded bg-blue-500 text-white">{"Search"}</Link<Route>>
                </div>
                if tvshows.loading {
                    <Loading classes="flex-col min-h-[200px]" />
                } else if tvshows.error.is_some() {
                    <ErrorMessage classes="min-h-[200px]" message="Couldn't load your TV shows..." />
                } else if let Some(shows) = &tvshows.data {
                    if shows.data.is_empty() {
                        <EmptyState title="No TV shows found" subtitle="Add some shows in order to see them here..." />
                    } else {
                        <div class="grid grid-cols-3 gap-4">
                            { for shows.data.iter().map(|show| html! {
                                <TVShowCardlet show={show.clone()} />
                            }) }
                        </div>
                    }
                } else {
                    <EmptyState title="No TV shows found" subtitle="Add some shows in order to see them here..." />
                }
            </main>
        </div>
    }
}

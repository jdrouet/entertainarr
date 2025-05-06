use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::component::header::Header;
use crate::component::tvshow_cardlet::TVShowCardlet;
use crate::component::tvshow_episode_cardlet::TVShowEpisodeCardlet;
use crate::hook::tvshow::use_followed_tvshows;
use crate::hook::tvshow_episode::use_episode_watchlist;

#[function_component(Home)]
pub fn home() -> Html {
    let tvshows = use_followed_tvshows();
    let episode_watchlist = use_episode_watchlist();

    html! {
        <div class="bg-gray-100 min-h-screen">
            <Header />
            <main class="max-w-6xl mx-auto p-6">
                <div class="flex flex-row justify-between items-center mb-4">
                    <h1 class="text-2xl font-bold text-gray-800">
                        {"Next episodes"}
                    </h1>
                </div>
                {
                    if episode_watchlist.loading {
                        html! {
                            <div class="flex justify-center py-16">
                                <div class="flex flex-col items-center space-y-2 text-gray-500">
                                    <svg class="animate-spin h-8 w-8 text-indigo-500" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8v8z"></path>
                                    </svg>
                                    <p class="text-sm mt-2">{"Loading watchlist..."}</p>
                                </div>
                            </div>
                        }
                    } else if let Some(err) = &episode_watchlist.error {
                        html! {
                            <div class="flex flex-col items-center justify-center text-center text-red-500 py-16 space-y-3">
                                <svg class="w-12 h-12 text-red-400" fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v2m0 4h.01M12 6a9 9 0 110 18 9 9 0 010-18z" />
                                </svg>
                                <h3 class="text-lg font-semibold">{"Oops! Something went wrong."}</h3>
                                <p class="text-sm text-red-400 max-w-md">
                                    { format!("We couldn’t load your watchlist: {}", err) }
                                </p>
                            </div>
                        }
                    } else if let Some(episodes) = &episode_watchlist.data {
                        if episodes.is_empty() {
                            html! {
                                <div class="flex flex-col items-center justify-center text-center text-gray-500 py-16">
                                    <svg class="w-16 h-16 mb-4 text-gray-300" fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M4 16h16M4 12h8m-8-4h16" />
                                    </svg>
                                    <p class="text-lg font-medium">{"You've seen them all"}</p>
                                    <p class="text-sm mt-1">
                                        <Link<Route> to={Route::TvshowSearch}>{"You are all up to date."}</Link<Route>>
                                    </p>
                                </div>
                            }
                        } else {
                            html! {
                                <div class="grid grid-cols-3 gap-4">
                                    { for episodes.iter().map(|episode| html! {
                                        <TVShowEpisodeCardlet episode={episode.clone()} />
                                    }) }
                                </div>
                            }
                        }
                    } else {
                        html! { <p>{"No shows found."}</p> }
                    }
                }
                <div class="flex flex-row justify-between items-center my-4">
                    <h1 class="text-2xl font-bold text-gray-800">
                        {"Followed TV Shows"}
                    </h1>
                    <Link<Route> to={Route::TvshowSearch} classes="text-sm px-4 py-2 rounded bg-blue-500 text-white">{"Search"}</Link<Route>>
                </div>
                {
                    if tvshows.loading {
                        html! {
                            <div class="flex justify-center py-16">
                                <div class="flex flex-col items-center space-y-2 text-gray-500">
                                    <svg class="animate-spin h-8 w-8 text-indigo-500" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8v8z"></path>
                                    </svg>
                                    <p class="text-sm mt-2">{"Loading TV shows..."}</p>
                                </div>
                            </div>
                        }
                    } else if let Some(err) = &tvshows.error {
                        html! {
                            <div class="flex flex-col items-center justify-center text-center text-red-500 py-16 space-y-3">
                                <svg class="w-12 h-12 text-red-400" fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v2m0 4h.01M12 6a9 9 0 110 18 9 9 0 010-18z" />
                                </svg>
                                <h3 class="text-lg font-semibold">{"Oops! Something went wrong."}</h3>
                                <p class="text-sm text-red-400 max-w-md">
                                    { format!("We couldn’t load your TV shows: {}", err) }
                                </p>
                            </div>
                        }
                    } else if let Some(shows) = &tvshows.data {
                        if shows.data.is_empty() {
                            html! {
                                <div class="flex flex-col items-center justify-center text-center text-gray-500 py-16">
                                    <svg class="w-16 h-16 mb-4 text-gray-300" fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M4 16h16M4 12h8m-8-4h16" />
                                    </svg>
                                    <p class="text-lg font-medium">{"No TV shows found"}</p>
                                    <p class="text-sm mt-1">
                                        <Link<Route> to={Route::TvshowSearch}>{"Add shows or check back later."}</Link<Route>>
                                    </p>
                                </div>
                            }
                        } else {
                            html! {
                                <div class="grid grid-cols-3 gap-4">
                                    { for shows.data.iter().map(|show| html! {
                                        <TVShowCardlet show={show.clone()} />
                                    }) }
                                </div>
                            }
                        }
                    } else {
                        html! { <p>{"No shows found."}</p> }
                    }
                }
            </main>
        </div>
    }
}

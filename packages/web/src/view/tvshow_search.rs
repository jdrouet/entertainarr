use std::sync::Arc;

use entertainarr_api::tvshow::TVShow;
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::component::header::Header;
use crate::component::tvshow_cardlet::TVShowCardlet;

async fn fetch_tvshows(query: String) -> Result<Vec<TVShow>, Arc<gloo_net::Error>> {
    let params = [("q", query)];
    let res = gloo_net::http::Request::get("/api/tvshows/search")
        .query(params)
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(Arc::new)?;
    res.json().await.map_err(Arc::new)
}

#[function_component(TVShowSearch)]
pub fn tvshow_search() -> Html {
    let query = use_search_param("q".into()).unwrap_or_default();
    let list = {
        let query = query.clone();
        use_async_with_options(fetch_tvshows(query), UseAsyncOptions::enable_auto())
    };

    html! {
        <div class="bg-gray-100 min-h-screen">
            <Header />
            <main class="max-w-4xl mx-auto p-6">
                <h2 class="text-2xl font-bold text-gray-800 mb-4">{"TV Shows - Search"}</h2>
                {
                    if list.loading {
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
                    } else if let Some(err) = &list.error {
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
                    } else if let Some(shows) = &list.data {
                        if shows.is_empty() {
                            html! {
                                <div class="flex flex-col items-center justify-center text-center text-gray-500 py-16">
                                    <svg class="w-16 h-16 mb-4 text-gray-300" fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M4 16h16M4 12h8m-8-4h16" />
                                    </svg>
                                    <p class="text-lg font-medium">{"No TV shows found"}</p>
                                    <p class="text-sm mt-1">{"Add shows or check back later."}</p>
                                </div>
                            }
                        } else {
                            html! {
                                <div class="grid gap-4">
                                    { for shows.iter().map(|show| html! {
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

use std::sync::Arc;

use entertainarr_api::tvshow::TVShow;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::hooks::use_navigator;

use crate::Route;
use crate::component::header::Header;
use crate::component::tvshow_cardlet::TVShowCardlet;

async fn fetch_tvshows(query: String) -> Result<Vec<TVShow>, Arc<gloo_net::Error>> {
    if query.is_empty() {
        return Ok(Vec::new());
    }
    let params = [("q", query)];
    let res = gloo_net::http::Request::get("/api/tvshows/search")
        .query(params)
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(Arc::new)?;
    res.json().await.map_err(Arc::new)
}

#[derive(Debug, serde::Serialize)]
struct QueryParams {
    q: String,
}

#[function_component(TVShowSearch)]
pub fn tvshow_search() -> Html {
    let navigator = use_navigator().unwrap();
    let query = use_search_param("q".into()).unwrap_or_default();
    let query_value = use_state(|| query.clone());
    let input = use_state(|| query.clone());

    let list = {
        let query_value = (*query_value).clone();
        use_async_with_options(fetch_tvshows(query_value), UseAsyncOptions::enable_auto())
    };

    let on_input = {
        let input = input.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(value) = e
                .target_dyn_into::<web_sys::HtmlInputElement>()
                .map(|e| e.value())
            {
                input.set(value);
            }
        })
    };

    let onsubmit = {
        let input = input.clone();
        let query_value = query_value.clone();
        let list = list.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let q = (*input).clone();
            query_value.set(q.clone());
            list.run();
            let _ = navigator.replace_with_query(&Route::TvshowSearch, &QueryParams { q });
        })
    };

    html! {
        <div class="bg-gray-100 min-h-screen">
            <Header />
            <main class="max-w-4xl mx-auto p-6">
                <h2 class="text-2xl font-bold text-gray-800 mb-4">{"TV Shows - Search"}</h2>
                <form class="flex mb-6 space-x-2" {onsubmit}>
                    <input
                        type="text"
                        value={(*input).clone()}
                        oninput={on_input}
                        class="flex-1 px-4 py-2 border rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-indigo-500"
                        placeholder="Search for TV shows..."
                    />
                    <button
                        class="flex items-center px-4 py-2 bg-indigo-600 text-white font-semibold rounded-md hover:bg-indigo-700 transition"
                        disabled={list.loading}
                        type="submit"
                    >
                        {
                            if list.loading {
                                html! {
                                    <svg class="animate-spin h-5 w-5 mr-2 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8v8z"></path>
                                    </svg>
                                }
                            } else {
                                html! { <span>{"Search"}</span> }
                            }
                        }
                    </button>
                </form>

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
                                    <p class="text-sm mt-1">{"Try updating the query."}</p>
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

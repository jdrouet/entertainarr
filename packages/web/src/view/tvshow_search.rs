use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::hooks::use_navigator;

use crate::Route;
use crate::component::empty_state::EmptyState;
use crate::component::error_message::ErrorMessage;
use crate::component::header::Header;
use crate::component::loading::Loading;
use crate::component::tvshow_cardlet::TVShowCardlet;

#[derive(Debug, serde::Serialize)]
struct QueryParams {
    q: String,
}

#[derive(Properties, PartialEq)]
struct SearchButtonProps {
    #[prop_or_default]
    loading: bool,
}

#[function_component(SearchButton)]
fn search_button(props: &SearchButtonProps) -> Html {
    html! {
        <button
            class="flex items-center px-4 py-2 bg-indigo-600 text-white font-semibold rounded-md hover:bg-indigo-700 transition"
            disabled={props.loading}
            type="submit"
        >
            if props.loading {
                <svg class="animate-spin h-5 w-5 mr-2 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                    <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8v8z"></path>
                </svg>
            } else {
                <span>{"Search"}</span>
            }
        </button>
    }
}

#[function_component(TVShowSearch)]
pub fn tvshow_search() -> Html {
    let navigator = use_navigator().unwrap();
    let query = use_search_param("q".into()).unwrap_or_default();
    let query_value = use_state(|| query.clone());
    let input = use_state(|| query.clone());

    let list = {
        let query_value = (*query_value).clone();
        use_async_with_options(
            crate::hook::tvshow::search(query_value),
            UseAsyncOptions::enable_auto(),
        )
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
            <main class="max-w-6xl mx-auto p-6">
                <h2 class="text-2xl font-bold text-gray-800 mb-4">{"TV Shows - Search"}</h2>
                <form class="flex mb-6 space-x-2" {onsubmit}>
                    <input
                        type="text"
                        value={(*input).clone()}
                        oninput={on_input}
                        class="flex-1 px-4 py-2 border rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-indigo-500"
                        placeholder="Search for TV shows..."
                    />
                    <SearchButton loading={list.loading} />
                </form>
                if list.loading {
                    <Loading classes="flex-col min-h-[400px]" />
                } else if list.error.is_some() {
                    <ErrorMessage classes="min-h-[400px]" message="Couldn't search for TV shows..." />
                } else if let Some(shows) = &list.data {
                    if shows.is_empty() {
                        <EmptyState classes="min-h-[400px]" title="No TV shows found" subtitle="Try updating the query..." />
                    } else {
                        <div class="grid grid-cols-3 gap-4">
                            { for shows.iter().map(|show| html! {
                                <TVShowCardlet show={show.clone()} />
                            }) }
                        </div>
                    }
                } else {
                    <EmptyState classes="min-h-[400px]" title="No TV shows found" subtitle="Try updating the query..." />
                }
            </main>
        </div>
    }
}

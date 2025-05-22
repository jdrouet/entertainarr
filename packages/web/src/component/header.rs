use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="bg-gray-900 text-white shadow-md">
            <div class="max-w-7xl mx-auto px-4 py-3 flex items-center justify-between">
                <Link<Route> to={Route::Home} classes="text-xl font-semibold tracking-wide">{"Entertainarr"}</Link<Route>>
                <nav class="space-x-4 text-sm">
                    <Link<Route> to={Route::Home} classes="hover:text-indigo-400 transition">{"Home"}</Link<Route>>
                    <Link<Route> to={Route::TvshowIndex} classes="hover:text-indigo-400 transition">{"TV Show"}</Link<Route>>
                    <a href="#" class="hover:text-indigo-400 transition">{"Library"}</a>
                    <a href="#" class="hover:text-indigo-400 transition">{"Settings"}</a>
                </nav>
            </div>
        </header>
    }
}

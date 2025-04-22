use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="bg-gray-900 text-white shadow-md">
            <div class="max-w-7xl mx-auto px-4 py-3 flex items-center justify-between">
                <h1 class="text-xl font-semibold tracking-wide">{"Entertainarr"}</h1>
                <nav class="space-x-4 text-sm">
                    <a href="#" class="hover:text-indigo-400 transition">{"Home"}</a>
                    <a href="#" class="hover:text-indigo-400 transition">{"Library"}</a>
                    <a href="#" class="hover:text-indigo-400 transition">{"Settings"}</a>
                </nav>
            </div>
        </header>
    }
}

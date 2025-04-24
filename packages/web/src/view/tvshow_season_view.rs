use yew::prelude::*;

use crate::component::header::Header;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub tvshow_id: u64,
    pub season_number: u64,
}

#[function_component(TVShowSeasonView)]
pub fn tvshow_view(_props: &Props) -> Html {
    html! {
        <div class="bg-gray-100 min-h-screen">
            <Header />
            <main class="max-w-6xl mx-auto px-4 py-8">
            </main>
        </div>
    }
}

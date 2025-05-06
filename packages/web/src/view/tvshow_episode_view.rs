use yew::prelude::*;

use crate::component::header::Header;
use crate::hook::tvshow::*;
use crate::hook::tvshow_episode::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub tvshow_id: u64,
    pub season_number: u64,
    pub episode_number: u64,
}

#[function_component(TVShowEpisodeView)]
pub fn tvshow_episode_view(props: &Props) -> Html {
    let tvshow = use_tvshow(props.tvshow_id);
    let episode = use_tvshow_episode(props.tvshow_id, props.season_number, props.episode_number);

    html! {
        <div class="bg-gray-100 min-h-screen">
            <Header />
            <main class="max-w-6xl mx-auto p-4">
                <video class="w-full" controls={true}>
                    <source src="/api/files/42" type="video/mp4" />
                </video>
                <div class="flex flex-row mb-1 items-center gap-4 my-4">
                    if let Some(ref tvshow) = tvshow.data {
                        <h1 class="text-2xl font-bold text-gray-900">
                            {format!("{} S{:02}E{:02}", tvshow.name, props.season_number, props.episode_number)}
                        </h1>
                    } else {
                        <div class="h-[32px] w-[300px] animate-pulse rounded-xl bg-gray-300"></div>
                    }
                    if let Some(air_date) = episode.data.as_ref().and_then(|item| item.air_date.map(|date| date.format("%Y-%m-%d").to_string())) {
                        <h3 class="text-md text-gray-600">
                            {air_date}
                        </h3>
                    }
                </div>
                if let Some(overview) = episode.data.as_ref().and_then(|v| v.overview.as_ref()) {
                    <div>{overview}</div>
                }
            </main>
        </div>
    }
}

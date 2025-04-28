use yew::prelude::*;

use crate::component::button::Button;
use crate::component::error_message::ErrorMessage;
use crate::component::header::Header;
use crate::component::loading::Loading;
use crate::component::tvshow_episode_list_item::TVShowEpisodeListItem;
use crate::hook::tvshow::use_tvshow;
use crate::hook::tvshow::use_tvshow_episodes;
use crate::hook::tvshow::use_tvshow_season;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub tvshow_id: u64,
    pub season_number: u64,
}

#[function_component(TVShowSeasonView)]
pub fn tvshow_season_view(props: &Props) -> Html {
    let tvshow = use_tvshow(props.tvshow_id);
    let season = use_tvshow_season(props.tvshow_id, props.season_number);
    let episodes = use_tvshow_episodes(props.tvshow_id, props.season_number);

    let tvshow_name = tvshow.inner.data.as_ref().map(|inner| inner.name.as_str());

    html! {
        <div class="bg-gray-100 min-h-screen">
            <Header />
            <main class="max-w-6xl mx-auto px-4 py-8">
                if let Some(err) = &season.error {
                    <ErrorMessage error={err.to_string()} />
                } else if let Some(season) = &season.data {
                    <div class="flex flex-col md:flex-row gap-6 mb-6">
                        if let Some(path) = season.poster_path.as_ref() {
                            <div class="w-full md:w-1/6">
                                <img
                                    class="max-h-[300px] mx-auto rounded-lg shadow-md"
                                    src={format!("https://image.tmdb.org/t/p/w500{path}")}
                                    alt={season.name.clone()}
                                />
                            </div>
                        }

                        <div class="flex-1">
                            <h1 class="text-3xl font-bold text-gray-900 mb-1">
                                if let Some(name) = tvshow_name {
                                    {name}
                                }
                            </h1>
                            <h2 class="text-2xl font-bold text-gray-800">
                                {season.name.as_str()}
                            </h2>
                            if let Some(date) = season.air_date {
                                <p class="text-sm text-gray-500 mt-1">
                                    { format!("Aired on {}", date.format("%B %d, %Y")) }
                                </p>
                            }
                            if let Some(overview) = &season.overview {
                                <p class="mt-4 text-gray-700">{ overview }</p>
                            }
                            <p class="text-sm text-gray-600 my-4">
                                { format!("{} episodes • {} watched", season.episode_count, season.watched_episode_count) }
                            </p>
                            <div>
                                <Button alt="Mark all episodes as watched" label="Watched" onclick={|_| {}} />
                            </div>
                        </div>
                    </div>
                } else {
                    <Loading />
                }

                if let Some(err) = &episodes.error {
                    <ErrorMessage error={err.to_string()} />
                } else if let Some(episodes) = &episodes.data {
                    <div class="space-y-4">
                        { for episodes.iter().map(|episode| {
                            html! {
                                <TVShowEpisodeListItem episode={episode.clone()} />
                            }
                        }) }
                    </div>
                } else {
                    <Loading />
                }
            </main>
        </div>
    }
}

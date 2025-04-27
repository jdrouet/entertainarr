use yew::prelude::*;

use crate::component::error_message::ErrorMessage;
use crate::component::header::Header;
use crate::component::loading::Loading;
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
                            <p class="text-sm text-gray-600 mt-4">
                                { format!("{} episodes • {} watched", season.episode_count, season.watched_episode_count) }
                            </p>
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
                            let is_watched = episode.watch.as_ref().map_or(false, |w| w.completed || w.progress > 0);
                            let watch_status = if is_watched {
                                html! { <span class="text-green-600 font-medium text-sm">{"Watched"}</span> }
                            } else {
                                html! { <span class="text-gray-500 text-sm">{"Not Watched"}</span> }
                            };

                            let file_status = if episode.file_count > 0 {
                                html! { <span class="text-blue-600 font-medium text-sm">{"Available"}</span> }
                            } else {
                                html! { <span class="text-red-500 font-medium text-sm">{"Not Available"}</span> }
                            };

                            let on_mark_watched = {
                                let episode_id = episode.id;
                                Callback::from(move |_| {
                                    // TODO: implement real mutation call
                                    web_sys::console::log_1(&format!("Mark episode {} as watched", episode_id).into());
                                })
                            };

                            html! {
                                <div class="bg-white rounded shadow p-4 flex flex-col md:flex-row md:justify-between md:items-start">
                                    <div class="flex-1 pr-4">
                                        <h3 class="text-lg font-semibold text-gray-800">
                                            { format!("Episode {}: {}", episode.episode_number, episode.name) }
                                        </h3>
                                        {
                                            if let Some(date) = &episode.air_date {
                                                html! {
                                                    <p class="text-sm text-gray-500 mt-1">
                                                        { format!("Aired: {}", date.format("%B %d, %Y")) }
                                                    </p>
                                                }
                                            } else {
                                                html!()
                                            }
                                        }
                                        {
                                            if let Some(overview) = &episode.overview {
                                                html! {
                                                    <p class="mt-2 text-sm text-gray-700">{ overview }</p>
                                                }
                                            } else {
                                                html!()
                                            }
                                        }
                                        <div class="mt-2 space-x-4">
                                            { watch_status }
                                            { file_status }
                                        </div>
                                    </div>
                                    <div class="mt-4 md:mt-0">
                                        <button
                                            onclick={on_mark_watched}
                                            class="bg-indigo-600 hover:bg-indigo-700 text-white text-sm font-semibold py-2 px-4 rounded shadow transition"
                                        >
                                            { "Mark as Watched" }
                                        </button>
                                    </div>
                                </div>
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

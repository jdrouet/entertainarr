use yew::prelude::*;

use crate::component::header::Header;
use crate::hook::tvshow::use_tvshow_episodes;
use crate::hook::tvshow::use_tvshow_season;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub tvshow_id: u64,
    pub season_number: u64,
}

#[function_component(TVShowSeasonView)]
pub fn tvshow_season_view(props: &Props) -> Html {
    let season = use_tvshow_season(props.tvshow_id, props.season_number);
    let episodes = use_tvshow_episodes(props.tvshow_id, props.season_number);

    if season.loading || episodes.loading {
        return html! {
            <div class="flex justify-center py-16">
                <div class="text-gray-500">{"Loading season data..."}</div>
            </div>
        };
    }

    if let Some(err) = &season.error {
        return html! {
            <div class="text-red-500 py-4">{ format!("Failed to load season: {}", err) }</div>
        };
    }

    if let Some(err) = &episodes.error {
        return html! {
            <div class="text-red-500 py-4">{ format!("Failed to load episodes: {}", err) }</div>
        };
    }

    let Some(season) = &season.data else {
        return html! {
            <div class="text-gray-500">{"Season not found."}</div>
        };
    };

    let Some(episodes) = &episodes.data else {
        return html! {
            <div class="text-gray-500">{"Episodes not found."}</div>
        };
    };

    html! {
        <div class="bg-gray-100 min-h-screen">
            <Header />
            <main class="max-w-5xl mx-auto px-4 py-6 space-y-8">
                <div class="bg-white rounded-lg shadow flex flex-col md:flex-row overflow-hidden">
                    {
                        if let Some(poster) = &season.poster_path {
                            html! {
                                <img
                                    src={format!("https://image.tmdb.org/t/p/w300{}", poster)}
                                    alt="Season Poster"
                                    class="w-full md:w-48 h-auto object-cover"
                                />
                            }
                        } else {
                            html! {
                                <div class="w-full md:w-48 h-72 bg-gray-200 flex items-center justify-center text-gray-400">
                                    {"No Image"}
                                </div>
                            }
                        }
                    }
                    <div class="p-6 flex flex-col justify-between">
                        <div>
                            <h2 class="text-2xl font-bold text-gray-800">
                                { format!("Season {} - {}", season.season_number, season.name) }
                            </h2>
                            {
                                if let Some(date) = season.air_date {
                                    html! {
                                        <p class="text-sm text-gray-500 mt-1">
                                            { format!("Aired on {}", date.format("%B %d, %Y")) }
                                        </p>
                                    }
                                } else {
                                    html!()
                                }
                            }
                            {
                                if let Some(overview) = &season.overview {
                                    html! {
                                        <p class="mt-4 text-gray-700">{ overview }</p>
                                    }
                                } else {
                                    html!()
                                }
                            }
                        </div>
                        <p class="text-sm text-gray-600 mt-4">
                            { format!("{} episodes • {} watched", season.episode_count, season.watched_episode_count) }
                        </p>
                    </div>
                </div>

                <div class="space-y-4">
                    { for episodes.iter().map(|episode| {
                        let is_watched = episode.watch.as_ref().map_or(false, |w| w.completed || w.progress > 0);
                        let watch_status = if is_watched {
                            html! { <span class="text-green-600 font-medium text-sm">{"Watched"}</span> }
                        } else {
                            html! { <span class="text-gray-500 text-sm">{"Not Watched"}</span> }
                        };

                        let file_status = if false {
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
            </main>
        </div>
    }
}

use yew::prelude::*;
use yew_hooks::{UseAsyncOptions, use_async_with_options};

use crate::component::header::Header;
use crate::component::loading::Loading;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub tvshow_id: u64,
}

#[function_component(TVShowView)]
pub fn tvshow_view(props: &Props) -> Html {
    let tvshow = use_async_with_options(
        crate::hook::tvshow::get_by_id(props.tvshow_id),
        UseAsyncOptions::enable_auto(),
    );

    html! {
        <div class="bg-gray-100 min-h-screen">
            <Header />
            <main class="max-w-6xl mx-auto px-4 py-8">
                {
                    if let Some(data) = &tvshow.data {
                        html! {
                            <div class="flex flex-col md:flex-row gap-6">
                                <img
                                    class="w-full md:w-1/3 rounded-lg shadow-md object-cover"
                                    src={format!(
                                        "https://image.tmdb.org/t/p/w500{}",
                                        data.poster_path.clone().unwrap_or_default()
                                    )}
                                    alt={data.name.clone()}
                                />

                                <div class="flex-1">
                                    <h1 class="text-3xl font-bold text-gray-900 mb-2">
                                        { &data.name }
                                        {
                                            if data.name != data.original_name {
                                                html! {
                                                    <span class="ml-2 text-sm text-gray-500">{
                                                        format!("({})", &data.original_name)
                                                    }</span>
                                                }
                                            } else {
                                                html! {}
                                            }
                                        }
                                    </h1>

                                    <div class="text-sm text-gray-600 space-y-1 mb-4">
                                        <div>{ format!("First Air Date: {}", data.first_air_date.map_or("N/A".to_string(), |d| d.to_string())) }</div>
                                        <div>{ format!("Original Language: {}", data.original_language.to_uppercase()) }</div>
                                        <div>{ format!("Origin Country: {}", data.origin_country.join(", ")) }</div>
                                        <div>{ format!("Vote Average: {:.1} / 10 ({} votes)", data.vote_average, data.vote_count) }</div>
                                        <div>{ format!("Popularity: {:.1}", data.popularity) }</div>
                                        { if data.adult {
                                            html! { <div class="text-red-600 font-semibold">{"Adult Content"}</div> }
                                        } else {
                                            html! {}
                                        }}
                                    </div>

                                    <p class="text-gray-800 leading-relaxed">
                                        { data.overview.clone().unwrap_or_else(|| "No description available.".to_string()) }
                                    </p>
                                </div>
                            </div>
                        }
                    } else if tvshow.loading {
                        html! { <Loading /> }
                    } else if let Some(err) = &tvshow.error {
                        html! { <div class="text-red-600">{ format!("Error: {}", err) }</div> }
                    } else {
                        html! {}
                    }
                }
            </main>
        </div>
    }
}

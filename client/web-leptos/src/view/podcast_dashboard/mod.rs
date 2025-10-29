use entertainarr_client_core::application::authenticated::podcast::dashboard::PodcastDashboardModel;
use leptos::prelude::*;

use crate::component::podcast_card::PodcastCard;

#[component]
pub fn View(model: PodcastDashboardModel) -> impl IntoView {
    view! {
        <crate::component::fullscreen::layout::FullscreenLayout>
            <h1>{"Podcasts"}</h1>
            <div>
                {model.data.into_iter().map(|item| {
                    view! {
                        <PodcastCard podcast={item} />
                    }
                }).collect_view()}
            </div>
        </crate::component::fullscreen::layout::FullscreenLayout>
    }
}

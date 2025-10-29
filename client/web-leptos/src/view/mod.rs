use entertainarr_client_core::application::ApplicationView;
use leptos::prelude::{Get, IntoAny};
use leptos::{IntoView, component, view};

use crate::context::core::use_view_model;

pub mod authentication;
pub mod home;
pub mod podcast_dashboard;
pub mod podcast_subscribe;

#[component]
pub fn RouterView() -> impl IntoView {
    let view_model = use_view_model();

    move || {
        let model = view_model.get();
        tracing::debug!("route = {:?}", model.route);
        tracing::debug!("view = {:?}", model.view);
        match model.view {
            ApplicationView::Authentication(view) => {
                view! {<authentication::View model=view />}.into_any()
            }
            ApplicationView::Home(view) => view! {<home::View model=view />}.into_any(),
            ApplicationView::Initialization => view! {<div />}.into_any(),
            ApplicationView::PodcastDashboard(view) => {
                view! {<podcast_dashboard::View model={view} />}.into_any()
            }
            ApplicationView::PodcastSubscribe(view) => {
                view! { <podcast_subscribe::View model=view /> }.into_any()
            }
        }
    }
}

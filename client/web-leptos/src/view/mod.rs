use entertainarr_client_core::View;
use leptos::prelude::{Get, IntoAny};
use leptos::{IntoView, component, view};

use crate::context::core::use_view_model;

pub mod authentication;
pub mod home;
pub mod podcast_subscription;

#[component]
pub fn RouterView() -> impl IntoView {
    let view_model = use_view_model();

    move || match view_model.get().view {
        View::Authentication(view) => view! {<authentication::View model=view />}.into_any(),
        View::Home(view) => view! {<home::View model=view />}.into_any(),
        View::Init(_view) => view! {<div />}.into_any(),
        View::PodcastSubscription(view) => {
            view! { <podcast_subscription::View model=view /> }.into_any()
        }
    }
}

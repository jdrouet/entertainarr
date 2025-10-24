use entertainarr_client_core::View;
use leptos::prelude::{Get, IntoAny};
use leptos::{IntoView, component, view};

use crate::context::core::use_view_model;

pub mod authentication;
pub mod home;

#[component]
pub fn RouterView() -> impl IntoView {
    let view_model = use_view_model();

    move || match view_model.get().view {
        View::Authentication(view) => {
            view! {<authentication::AuthenticationView model=view />}.into_any()
        }
        View::Home(view) => view! {<home::HomeView model=view />}.into_any(),
        View::Init(_view) => view! {<div />}.into_any(),
    }
}

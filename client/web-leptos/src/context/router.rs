use std::borrow::Cow;

use entertainarr_client_core::application::{ApplicationEvent, router::Route};
use leptos::prelude::*;
use leptos_router::{
    NavigateOptions,
    hooks::{use_navigate, use_url},
};

use crate::context::core::{use_events, use_view_model};

fn path(route: Route) -> Cow<'static, str> {
    match route {
        Route::Home => Cow::Borrowed("#/"),
        Route::Authentication => Cow::Borrowed("#/authentication"),
        Route::PodcastDashboard => Cow::Borrowed("#/podcasts"),
        Route::PodcastSubscribe => Cow::Borrowed("#/podcasts/subscribe"),
    }
}

pub(crate) fn parse_route(path: &str) -> Route {
    match path {
        "" | "#/" => Route::Home,
        "#/authentication" => Route::Authentication,
        "#/podcasts" => Route::PodcastDashboard,
        "#/podcasts/subscribe" => Route::PodcastSubscribe,
        _ => Route::Home,
    }
}

#[component]
pub fn RouterManager() -> impl IntoView {
    let url = use_url();
    let navigate = use_navigate();
    let view = use_view_model();
    let (changing, set_changing) = signal(false);
    let (_, set_event) = use_events();

    Effect::new(move |_| {
        let is_changing = changing.get();
        let expected = view.with(|model| model.route.clone());
        let current = url.with(|url| super::router::parse_route(url.hash()));
        if is_changing && current == expected {
            set_changing.set(false);
        }
        if !is_changing && current != expected {
            set_changing.set(true);
            navigate(path(expected).as_ref(), NavigateOptions::default())
        }
    });

    Effect::new(move |_| {
        let route = url.with(|url| super::router::parse_route(url.hash()));
        set_event.set(ApplicationEvent::RouteChange(route));
    });

    view! {<></>}
}

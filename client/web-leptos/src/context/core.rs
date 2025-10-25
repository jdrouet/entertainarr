use entertainarr_client_core::application::ApplicationViewModel;
use entertainarr_client_core::application::InitializationEvent;
use entertainarr_client_core::application::router::Route;
use leptos::prelude::{
    Children, Effect, Get, ReadSignal, WithUntracked, WriteSignal, expect_context, provide_context,
    signal,
};
use leptos::{IntoView, component};
use leptos_router::hooks::use_url;

fn path_to_route(path: &str) -> Route {
    match path {
        "/authentication" => Route::Authentication,
        _ => Route::Home,
    }
}

#[component]
pub fn CoreContext(children: Children) -> impl IntoView {
    let core = crate::core::new();

    let url = use_url();
    let base_url = url.with_untracked(|value| value.origin().to_string());
    let _route = url.with_untracked(|value| path_to_route(value.path()));

    let (view, render) = signal(core.view());
    let (event, set_event) = signal(
        InitializationEvent {
            server_url: base_url,
            authentication_token: crate::service::storage::get_local_storage(
                "authentication-token",
            ),
            route: Some(Route::Home),
        }
        .into(),
    );

    provide_context((event.clone(), set_event));
    provide_context(view);

    Effect::new(move |_| {
        let current_event = event.get();
        crate::core::update(&core, current_event, render);
    });

    children()
}

pub fn use_view_model() -> ReadSignal<ApplicationViewModel> {
    expect_context()
}

pub fn use_events() -> (
    ReadSignal<entertainarr_client_core::application::ApplicationEvent>,
    WriteSignal<entertainarr_client_core::application::ApplicationEvent>,
) {
    expect_context()
}

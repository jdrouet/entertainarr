use entertainarr_client_core::application::ApplicationViewModel;
use entertainarr_client_core::application::InitializationEvent;
use leptos::prelude::{
    Children, Effect, Get, ReadSignal, WithUntracked, WriteSignal, expect_context, provide_context,
    signal,
};
use leptos::{IntoView, component};
use leptos_router::hooks::use_url;

#[component]
pub(crate) fn CoreContext(children: Children) -> impl IntoView {
    let core = crate::core::new();

    let url = use_url();
    let base_url = url.with_untracked(|value| value.origin().to_string());
    let route = url.with_untracked(|url| super::router::parse_route(url.hash()));

    let (view, render) = signal(core.view());
    let (event, set_event) = signal(
        InitializationEvent {
            server_url: base_url,
            authentication_token: crate::service::storage::get_local_storage(
                "authentication-token",
            ),
            route: Some(route),
        }
        .into(),
    );

    provide_context((event, set_event));
    provide_context(view);

    Effect::new(move |_| {
        let current_event = event.get();
        crate::core::update(&core, current_event, render);
    });

    children()
}

pub(crate) fn use_view_model() -> ReadSignal<ApplicationViewModel> {
    expect_context()
}

pub(crate) fn use_events() -> (
    ReadSignal<entertainarr_client_core::application::ApplicationEvent>,
    WriteSignal<entertainarr_client_core::application::ApplicationEvent>,
) {
    expect_context()
}

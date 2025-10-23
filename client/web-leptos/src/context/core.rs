use entertainarr_client_core::domain::init::InitEvent;
use entertainarr_client_core::{Event, ViewModel};
use leptos::prelude::{
    Children, Effect, Get, ReadSignal, WithUntracked, WriteSignal, expect_context, provide_context,
    signal,
};
use leptos::{IntoView, component};
use leptos_router::hooks::use_url;

#[component]
pub fn CoreContext(children: Children) -> impl IntoView {
    let core = crate::core::new();

    let url = use_url();
    let base_url = url.with_untracked(|value| value.origin().to_string());

    let (view, render) = signal(core.view());
    let (event, set_event) = signal(Event::Init(InitEvent {
        server_url: base_url,
        authentication_token: crate::service::storage::get_local_storage("authentication-token"),
    }));

    provide_context((event.clone(), set_event));
    provide_context(view);

    Effect::new(move |_| {
        let current_event = event.get();
        tracing::warn!(?current_event, "event updated");
        crate::core::update(&core, current_event, render);
    });

    Effect::new(move |_| {
        let current_view = view.get();
        tracing::warn!(?current_view, "view updated");
    });

    children()
}

pub fn use_view_model() -> ReadSignal<ViewModel> {
    expect_context()
}

pub fn use_events() -> (ReadSignal<Event>, WriteSignal<Event>) {
    expect_context()
}

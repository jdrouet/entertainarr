use entertainarr_client_core::{Event, init::InitEvent};
use leptos::prelude::*;
use leptos_router::hooks::use_url;

mod core;
mod service;
mod view;

#[component]
pub fn RootComponent() -> impl IntoView {
    let core = core::new();

    let url = use_url();
    let base_url = url.with_untracked(|value| value.origin().to_string());

    let (_view, render) = signal(core.view());
    let (event, set_event) = signal(Event::Init(InitEvent {
        server_url: base_url,
        authentication_token: service::storage::get_local_storage("authentication-token"),
    }));

    Effect::new(move |_| {
        core::update(&core, event.get(), render);
    });

    view! {
        <view::authentication::AuthenticationView on_change=set_event />
    }
}

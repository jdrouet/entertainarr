use std::rc::Rc;

use entertainarr_client_core::{
    Application, Effect, Event, HttpResult, ViewModel, authentication::AuthenticationEvent,
};
use leptos::prelude::{Update as _, WriteSignal};

pub type Core = Rc<entertainarr_client_core::Core<Application>>;

pub fn new() -> Core {
    Rc::new(entertainarr_client_core::Core::new())
}

pub fn update(core: &Core, event: Event, render: WriteSignal<ViewModel>) {
    tracing::info!(?event, "core::update");
    for effect in core.process_event(event) {
        process_effect(core, effect, render);
    }
}

pub fn process_effect(core: &Core, effect: Effect, render: WriteSignal<ViewModel>) {
    tracing::info!(?effect, "core::process_effect");
    match effect {
        Effect::Render(_) => {
            render.update(|view| *view = core.view());
        }
        Effect::Http(req) => {
            tracing::warn!(?req, "http request");
            let event = Event::Authentication(AuthenticationEvent::Callback(HttpResult::Err(
                crux_http::HttpError::Io("SOMETHING WENT WRONG".into()),
            )));
            update(core, event, render);
        }
    }
}

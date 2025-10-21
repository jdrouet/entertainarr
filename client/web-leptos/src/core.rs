use std::rc::Rc;

use entertainarr_client_core::{
    Application, Effect, Event, ViewModel, capability::persistence::Persistence,
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
        Effect::Http(mut request) => {
            leptos::task::spawn_local({
                let core = core.clone();

                async move {
                    let response = crate::service::http::request(&request.operation).await;

                    for effect in core
                        .resolve(&mut request, response.into())
                        .expect("should resolve")
                    {
                        process_effect(&core, effect, render);
                    }
                }
            });
        }
        Effect::Persistence(req) => match req.operation {
            Persistence::Store(req) => {
                crate::service::storage::set_local_storage(req.key.as_str(), req.value.as_str());
            }
        },
        Effect::Render(_) => {
            render.update(|view| *view = core.view());
        }
    }
}

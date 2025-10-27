use std::sync::Arc;

use entertainarr_client_core::Application;
use entertainarr_client_core::application::{ApplicationEvent, ApplicationViewModel};
use entertainarr_client_core::effect::Effect;
use entertainarr_client_core::effect::persistence::Persistence;
use leptos::prelude::{Update, WriteSignal};

pub type Core = Arc<entertainarr_client_core::Core<Application>>;

pub fn new() -> Core {
    Arc::new(entertainarr_client_core::Core::new())
}

pub fn update(core: &Core, event: ApplicationEvent, render: WriteSignal<ApplicationViewModel>) {
    for effect in core.process_event(event) {
        process_effect(core, effect, render);
    }
}

pub fn process_effect(core: &Core, effect: Effect, render: WriteSignal<ApplicationViewModel>) {
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
            Persistence::Clear(req) => {
                crate::service::storage::remove_local_storage(req.key.as_str());
            }
        },
        Effect::Render(_) => {
            render.update(|view| *view = core.view());
        }
    }
}

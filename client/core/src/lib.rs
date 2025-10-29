use crux_core::Command;
pub use crux_core::Core;

pub mod application;
pub mod effect;
pub mod entity;

pub type ApplicationCommand = Command<crate::effect::Effect, crate::application::ApplicationEvent>;

#[derive(Default)]
pub struct Application;

impl crux_core::App for Application {
    type Capabilities = ();
    type Effect = crate::effect::Effect;
    type Event = crate::application::ApplicationEvent;
    type Model = crate::application::ApplicationModel;
    type ViewModel = crate::application::ApplicationViewModel;

    fn update(
        &self,
        event: Self::Event,
        model: &mut Self::Model,
        _caps: &(),
    ) -> Command<Self::Effect, Self::Event> {
        tracing::info!(event = %event.name(), "handle event");
        model.update(event)
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        model.view()
    }
}

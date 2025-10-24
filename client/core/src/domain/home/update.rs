use crux_core::{Command, render::render};

impl crate::Application {
    pub fn update_home(
        &self,
        event: super::HomeEvent,
        _root: &mut crate::Model,
    ) -> Command<crate::Effect, crate::Event> {
        match event {
            super::HomeEvent::Initialize => render(),
        }
    }
}

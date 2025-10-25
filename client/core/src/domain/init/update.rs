use crux_core::{Command, render::render};

use crate::domain::{AuthenticatedModel, home::Event};

impl crate::Application {
    pub fn update_init(
        &self,
        event: super::Event,
        model: &mut crate::Model,
    ) -> Command<crate::Effect, crate::Event> {
        // if not on initializing state, ignore events
        if !matches!(model, crate::Model::Initializing) {
            return render();
        }

        match event.authentication_token {
            Some(authentication_token) => {
                let next = crate::Model::Authenticated {
                    authentication_token,
                    model: AuthenticatedModel::default(),
                    server_url: event.server_url,
                };
                let _ = std::mem::replace(model, next);
                Command::all([Command::event(Event::Initialize.into()), render()])
            }
            None => {
                let next = crate::Model::Authentication {
                    model: Default::default(),
                    server_url: event.server_url,
                };
                let _ = std::mem::replace(model, next);
                render()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::init::Event;

    #[test]
    fn should_ignore_events_if_initialized() {
        let app = crate::Application;
        let mut root = crate::Model::Authenticated {
            authentication_token: String::from("DATA"),
            model: Default::default(),
            server_url: String::from("DATA"),
        };
        let mut res = app.update_init(
            Event {
                server_url: "http://localhost:1234".into(),
                authentication_token: None,
            },
            &mut root,
        );
        let crate::Model::Authenticated {
            authentication_token,
            model: _,
            server_url,
        } = &root
        else {
            panic!("should be in authenticated state");
        };
        assert_eq!(authentication_token, "DATA");
        assert_eq!(server_url, "DATA");
        let mut effects: Vec<_> = res.effects().collect();
        assert_eq!(effects.len(), 1);
        let effect = effects.pop().unwrap();
        assert!(effect.is_render());
    }

    #[test]
    fn should_update_model_without_authenticated_token() {
        let app = crate::Application;
        let mut root = crate::Model::Initializing;
        let mut res = app.update_init(
            Event {
                server_url: "http://localhost:1234".into(),
                authentication_token: None,
            },
            &mut root,
        );
        let crate::Model::Authentication { model, server_url } = &root else {
            panic!("should be authentication state");
        };
        assert_eq!(server_url, "http://localhost:1234");
        assert!(!model.loading);
        assert!(model.error.is_none());
        let mut effects: Vec<_> = res.effects().collect();
        assert_eq!(effects.len(), 1);
        let effect = effects.pop().unwrap();
        assert!(effect.is_render());
    }

    #[test]
    fn should_update_model_with_authenticated_token() {
        let app = crate::Application;
        let mut root = crate::Model::Initializing;
        let mut res = app.update_init(
            Event {
                server_url: "http://localhost:1234".into(),
                authentication_token: Some("token".into()),
            },
            &mut root,
        );
        let crate::Model::Authenticated {
            authentication_token,
            model: _,
            server_url,
        } = &root
        else {
            panic!("should be authenticated state");
        };
        assert_eq!(authentication_token, "token");
        assert_eq!(server_url, "http://localhost:1234");
        let mut effects: Vec<_> = res.effects().collect();
        assert_eq!(effects.len(), 1);
        let effect = effects.pop().unwrap();
        assert!(effect.is_render());
    }
}

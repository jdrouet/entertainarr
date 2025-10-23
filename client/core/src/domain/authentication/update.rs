use crux_core::{Command, render::render};

impl crate::Application {
    pub fn update_authentication(
        &self,
        event: super::AuthenticationEvent,
        model: &mut crate::Model,
    ) -> Command<crate::Effect, crate::Event> {
        match event {
            super::AuthenticationEvent::Request(req) => {
                let crate::Model::Authentication { model, server_url } = model else {
                    return render();
                };

                model.loading = true;
                Command::all([req.execute(&server_url), render()])
            }
            super::AuthenticationEvent::Error(err) => {
                let crate::Model::Authentication {
                    model,
                    server_url: _,
                } = model
                else {
                    return render();
                };

                model.loading = false;
                model.error = Some(err);
                render()
            }
            super::AuthenticationEvent::Success(authentication_token) => {
                if let Some(server_url) = model.server_url().map(String::from) {
                    let _ = std::mem::replace(
                        model,
                        crate::Model::Authenticated {
                            authentication_token,
                            server_url,
                        },
                    );
                }
                render()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::authentication::{
        AuthenticationError, AuthenticationEvent, AuthenticationKind, AuthenticationModel,
        AuthenticationRequest,
    };

    #[test]
    fn request_should_be_ignored_if_not_authentication() {
        let app = crate::Application;
        let mut root = crate::Model::Initializing;
        let mut res = app.update_authentication(
            AuthenticationEvent::Request(AuthenticationRequest {
                email: "user@example.com".into(),
                password: "password".into(),
                kind: AuthenticationKind::Login,
            }),
            &mut root,
        );
        let crate::Model::Initializing = &root else {
            panic!("should be init state");
        };
        let mut effects: Vec<_> = res.effects().collect();
        assert_eq!(effects.len(), 1);
        let effect = effects.pop().unwrap();
        assert!(effect.is_render());
    }

    #[test]
    fn request_should_update_and_trigger_events() {
        let app = crate::Application;
        let mut root = crate::Model::Authentication {
            model: Default::default(),
            server_url: String::from("http://server"),
        };
        let mut res = app.update_authentication(
            AuthenticationEvent::Request(AuthenticationRequest {
                email: "user@example.com".into(),
                password: "password".into(),
                kind: AuthenticationKind::Login,
            }),
            &mut root,
        );
        let crate::Model::Authentication { model, server_url } = &root else {
            panic!("should be authentication state");
        };
        assert_eq!(server_url, "http://server");
        assert!(model.loading);
        assert!(model.error.is_none());
        let mut effects: Vec<_> = res.effects().collect();
        assert_eq!(effects.len(), 2);
        let effect = effects.pop().unwrap();
        assert!(effect.is_render());
        let effect = effects.pop().unwrap();
        assert!(effect.is_http());
    }

    #[test]
    fn error_should_reset_loading() {
        let app = crate::Application;
        let mut root = crate::Model::Authentication {
            model: AuthenticationModel {
                loading: true,
                error: None,
            },
            server_url: String::from("http://server"),
        };
        let mut res = app.update_authentication(
            AuthenticationEvent::Error(AuthenticationError::EmailConflict),
            &mut root,
        );
        let crate::Model::Authentication { model, server_url } = &root else {
            panic!("should be authentication state");
        };
        assert_eq!(server_url, "http://server");
        assert!(!model.loading);
        let error = model.error.as_ref().unwrap();
        assert!(matches!(error, AuthenticationError::EmailConflict));
        let mut effects: Vec<_> = res.effects().collect();
        assert_eq!(effects.len(), 1);
        let effect = effects.pop().unwrap();
        assert!(effect.is_render());
    }
}

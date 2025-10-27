use crux_core::render::render;

use crate::{
    ApplicationCommand,
    application::{ApplicationState, router::Route},
    effect::persistence::Persistence,
};

mod execute;

#[derive(Clone, Debug, Eq, PartialEq, facet::Facet, serde::Serialize, serde::Deserialize)]
#[repr(C)]
pub enum AuthenticationError {
    EmailConflict,
    EmailTooShort,
    PasswordTooShort,
    InvalidCredentials,
    Network,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct AuthenticationModel {
    pub loading: bool,
    pub error: Option<AuthenticationError>,
}

#[derive(Clone, Debug, Eq, PartialEq, facet::Facet, serde::Serialize, serde::Deserialize)]
#[repr(C)]
pub enum AuthenticationEvent {
    Submit(AuthenticationRequest),
    Success(AuthenticationSuccess),
    Error(AuthenticationError),
    Logout,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, facet::Facet, serde::Serialize, serde::Deserialize)]
#[repr(C)]
pub enum AuthenticationKind {
    Login,
    Signup,
}

#[derive(Clone, Debug, Eq, PartialEq, facet::Facet, serde::Serialize, serde::Deserialize)]
#[repr(C)]
pub struct AuthenticationRequest {
    pub email: String,
    pub password: String,
    pub kind: AuthenticationKind,
}

#[derive(Clone, Debug, Eq, PartialEq, facet::Facet, serde::Serialize, serde::Deserialize)]
#[repr(C)]
pub struct AuthenticationSuccess {
    pub token: String,
}

impl crate::application::ApplicationModel {
    pub fn handle_authentication_event(
        &mut self,
        event: AuthenticationEvent,
    ) -> crate::ApplicationCommand {
        let Some(server_url) = self.server_url.as_deref() else {
            return render();
        };
        if let AuthenticationEvent::Logout = event {
            return ApplicationCommand::all([
                Route::Authentication.into(),
                Persistence::clear("authentication-token"),
                render(),
            ]);
        };
        match &mut self.state {
            ApplicationState::Authentication(model) => match event {
                AuthenticationEvent::Submit(req) => {
                    model.loading = true;
                    ApplicationCommand::all([req.execute(server_url), render()])
                }
                AuthenticationEvent::Success(res) => {
                    model.loading = false;
                    self.session = Some(super::session::Session {
                        token: res.token.clone(),
                    });
                    ApplicationCommand::all([
                        Route::Home.into(),
                        Persistence::store("authentication-token", res.token),
                        render(),
                    ])
                }
                AuthenticationEvent::Error(err) => {
                    model.loading = false;
                    model.error = Some(err);
                    render()
                }
                AuthenticationEvent::Logout => render(),
            },
            _ => render(),
        }
    }
}

use crux_core::render::render;

pub mod authenticated;
pub mod authentication;
pub mod router;
pub mod session;

#[derive(Debug)]
pub struct ApplicationModel {
    route: router::Route,
    session: Option<session::Session>,
    state: ApplicationState,
    server_url: Option<String>,
}

impl Default for ApplicationModel {
    fn default() -> Self {
        Self {
            route: router::Route::Authentication,
            session: None,
            state: ApplicationState::Initialization,
            server_url: None,
        }
    }
}

#[derive(Debug)]
pub(crate) enum ApplicationState {
    Initialization,
    Authentication(authentication::AuthenticationModel),
    Authenticated(authenticated::AuthenticatedModel),
}

impl ApplicationState {
    pub fn on_mount(&self) -> crate::ApplicationCommand {
        match self {
            Self::Initialization | Self::Authentication(_) => render(),
            Self::Authenticated(inner) => inner.on_mount(), // TODO
        }
    }

    fn view(&self) -> ApplicationView {
        match self {
            Self::Initialization => ApplicationView::Initialization,
            Self::Authentication(inner) => ApplicationView::Authentication(inner.clone()),
            Self::Authenticated(authenticated::AuthenticatedModel::Home(inner)) => {
                ApplicationView::Home(inner.clone())
            }
            Self::Authenticated(authenticated::AuthenticatedModel::PodcastSubscribe(inner)) => {
                ApplicationView::PodcastSubscribe(inner.clone())
            }
        }
    }
}

#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    derive_more::From,
    facet::Facet,
    serde::Serialize,
    serde::Deserialize,
)]
#[repr(C)]
pub enum ApplicationEvent {
    Authentication(authentication::AuthenticationEvent),
    Authenticated,
    Home(authenticated::home::HomeEvent),
    Initialization(InitializationEvent),
    Noop, // does nothing
    PodcastSubscribe(authenticated::podcast::subscribe::PodcastSubscribeEvent),
    RouteChange(router::Route),
}

#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    derive_more::From,
    facet::Facet,
    serde::Serialize,
    serde::Deserialize,
)]
#[repr(C)]
pub struct InitializationEvent {
    pub server_url: String,
    pub authentication_token: Option<String>,
    pub route: Option<self::router::Route>,
}

impl ApplicationModel {
    fn handle_initialization_event(
        &mut self,
        event: InitializationEvent,
    ) -> crate::ApplicationCommand {
        self.server_url = Some(event.server_url);
        if let Some(token) = event.authentication_token {
            self.session = Some(self::session::Session { token });
        }
        self.handle_router_event(event.route.unwrap_or(router::Route::Home))
    }

    pub(crate) fn update(&mut self, event: ApplicationEvent) -> crate::ApplicationCommand {
        if self.server_url.is_none() {
            return if let ApplicationEvent::Initialization(inner) = event {
                self.handle_initialization_event(inner)
            } else {
                render()
            };
        }

        match event {
            ApplicationEvent::Authentication(event) => self.handle_authentication_event(event),
            ApplicationEvent::Home(event) => self.handle_home_event(event),
            ApplicationEvent::Initialization(_) => render(),
            ApplicationEvent::Authenticated => render(),
            ApplicationEvent::PodcastSubscribe(event) => self.handle_podcast_subscribe_event(event),
            ApplicationEvent::RouteChange(route) => self.handle_router_event(route),
            ApplicationEvent::Noop => render(),
        }
    }
}

impl ApplicationModel {
    pub(crate) fn view(&self) -> ApplicationViewModel {
        ApplicationViewModel {
            route: self.route.clone(),
            view: self.state.view(),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct ApplicationViewModel {
    pub route: self::router::Route,
    pub view: ApplicationView,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum ApplicationView {
    Initialization,
    Authentication(self::authentication::AuthenticationModel),
    Home(self::authenticated::home::HomeModel),
    PodcastSubscribe(self::authenticated::podcast::subscribe::PodcastSubscribeModel),
}

#[cfg(test)]
mod tests {
    use crate::application::authenticated::AuthenticatedModel;

    use super::{ApplicationState, InitializationEvent, router::Route};

    #[test]
    fn should_route_to_authentication() {
        let mut model = crate::application::ApplicationModel::default();
        let mut cmd = model.update(crate::application::ApplicationEvent::Initialization(
            InitializationEvent {
                server_url: "http://localhost".into(),
                authentication_token: None,
                route: None,
            },
        ));
        assert_eq!(model.route, Route::Authentication);
        assert!(matches!(model.state, ApplicationState::Authentication(_)));
        let events: Vec<_> = cmd.events().collect();
        assert!(events.is_empty());
        let mut effects: Vec<_> = cmd.effects().collect();
        assert_eq!(effects.len(), 1);
        let effect = effects.pop().unwrap();
        assert!(effect.is_render());
    }

    #[test]
    fn should_route_to_home() {
        let mut model = crate::application::ApplicationModel::default();
        let mut cmd = model.update(crate::application::ApplicationEvent::Initialization(
            InitializationEvent {
                server_url: "http://localhost".into(),
                authentication_token: Some("token".into()),
                route: Some(Route::Home),
            },
        ));
        assert_eq!(model.route, Route::Home);
        assert!(matches!(
            model.state,
            ApplicationState::Authenticated(AuthenticatedModel::Home(_))
        ));
        let events: Vec<_> = cmd.events().collect();
        assert!(events.is_empty());
        let mut effects: Vec<_> = cmd.effects().collect();
        assert_eq!(effects.len(), 1);
        let effect = effects.pop().unwrap();
        assert!(effect.is_render());
    }
}

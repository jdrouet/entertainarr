use crux_core::render::render;

use crate::effect::http::Operation;

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
            Self::Authenticated(authenticated::AuthenticatedModel::PodcastDashboard(inner)) => {
                ApplicationView::PodcastDashboard(inner.clone())
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
    PodcastDashboard(authenticated::podcast::dashboard::PodcastDashboardEvent),
    PodcastSubscribe(authenticated::podcast::subscribe::PodcastSubscribeEvent),
    RouteChange(router::Route),
}

impl ApplicationEvent {
    pub fn name(&self) -> &'static str {
        use crate::application::authenticated::home::HomeEvent;
        use crate::application::authenticated::podcast::dashboard::PodcastDashboardEvent;
        use crate::application::authenticated::podcast::subscribe::PodcastSubscribeEvent;
        use crate::application::authentication::AuthenticationEvent;

        match self {
            Self::Authenticated => "authenticated",
            Self::Authentication(AuthenticationEvent::Submit(_)) => "authentication.submit",
            Self::Authentication(AuthenticationEvent::Success(_)) => "authentication.success",
            Self::Authentication(AuthenticationEvent::Error(_)) => "authentication.error",
            Self::Authentication(AuthenticationEvent::Logout) => "authentication.logout",
            Self::Home(HomeEvent::ListPodcastEpisodesRequest) => {
                "authenticated.home.list-podcast-episodes.request"
            }
            Self::Home(HomeEvent::ListPodcastEpisodesSuccess(_)) => {
                "authenticated.home.list-podcast-episodes.success"
            }
            Self::Home(HomeEvent::ListPodcastEpisodesError(_)) => {
                "authenticated.home.list-podcast-episodes.error"
            }
            Self::Initialization(_) => "initialization",
            Self::Noop => "noop",
            Self::PodcastDashboard(PodcastDashboardEvent::ListPodcastSubscription(
                Operation::Request(_),
            )) => "authenticated.podcast-dashboard.list-podcast-subscription.request",
            Self::PodcastDashboard(PodcastDashboardEvent::ListPodcastSubscription(
                Operation::Success(_),
            )) => "authenticated.podcast-dashboard.list-podcast-subscription.success",
            Self::PodcastDashboard(PodcastDashboardEvent::ListPodcastSubscription(
                Operation::Error(_),
            )) => "authenticated.podcast-dashboard.list-podcast-subscription.error",
            Self::PodcastSubscribe(PodcastSubscribeEvent::Submit(_)) => {
                "authenticated.podcast-subscribe.submit"
            }
            Self::PodcastSubscribe(PodcastSubscribeEvent::Success) => {
                "authenticated.podcast-subscribe.success"
            }
            Self::PodcastSubscribe(PodcastSubscribeEvent::Error(_)) => {
                "authenticated.podcast-subscribe.error"
            }
            Self::RouteChange(router::Route::Authentication) => "route.change.authentication",
            Self::RouteChange(router::Route::Home) => "route.change.home",
            Self::RouteChange(router::Route::PodcastDashboard) => "route.change.podcast-dashboard",
            Self::RouteChange(router::Route::PodcastSubscribe) => "route.change.podcast-subscribe",
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
            ApplicationEvent::Home(authenticated::home::HomeEvent::ListPodcastEpisodesError(
                ref err,
            )) if err.is_token_expired() => {
                return crate::ApplicationCommand::event(
                    authentication::AuthenticationEvent::Logout.into(),
                );
            }
            _ => {}
        }

        match event {
            ApplicationEvent::Authentication(event) => self.handle_authentication_event(event),
            ApplicationEvent::Home(event) => self.handle_home_event(event),
            ApplicationEvent::Initialization(_) => render(),
            ApplicationEvent::Authenticated => render(),
            ApplicationEvent::PodcastDashboard(event) => self.handle_podcast_dashboard_event(event),
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
    PodcastDashboard(self::authenticated::podcast::dashboard::PodcastDashboardModel),
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
        assert_eq!(events.len(), 1);
        let effects: Vec<_> = cmd.effects().collect();
        assert!(effects.is_empty());
    }
}

use crux_core::{Command, render::render};

use crate::application::ApplicationState;

#[derive(Clone, Debug, Eq, PartialEq, facet::Facet, serde::Serialize, serde::Deserialize)]
#[repr(C)]
pub enum Route {
    Authentication,
    Home,
    PodcastSubscribe,
    PodcastDashboard,
}

impl Route {
    pub const fn requires_authentication(&self) -> bool {
        matches!(self, Self::Home)
    }
}

impl From<Route> for crate::ApplicationCommand {
    fn from(value: Route) -> Self {
        Command::event(value.into())
    }
}

impl super::ApplicationModel {
    fn update_route(&mut self, route: Route) {
        self.route = route;
        match self.route {
            Route::Authentication => {
                self.state = ApplicationState::Authentication(Default::default());
            }
            Route::Home => {
                self.state = ApplicationState::Authenticated(
                    super::authenticated::AuthenticatedModel::Home(Default::default()),
                );
            }
            Route::PodcastDashboard => {
                self.state = ApplicationState::Authenticated(
                    super::authenticated::AuthenticatedModel::PodcastDashboard(Default::default()),
                );
            }
            Route::PodcastSubscribe => {
                self.state = ApplicationState::Authenticated(
                    super::authenticated::AuthenticatedModel::PodcastSubscribe(Default::default()),
                );
            }
        }
    }

    fn handle_router_change(
        &mut self,
        authenticated: bool,
        route: Route,
    ) -> crate::ApplicationCommand {
        if self.route == route {
            return render();
        }
        if route.requires_authentication() && !authenticated {
            self.update_route(Route::Authentication)
        } else {
            self.update_route(route)
        };
        self.state.on_mount()
    }

    pub(crate) fn handle_router_event(&mut self, route: Route) -> crate::ApplicationCommand {
        let authenticated = self.session.is_some();
        self.handle_router_change(authenticated, route)
    }
}

use crux_core::render::render;

use crate::{
    application::{ApplicationState, authenticated::AuthenticatedModel},
    effect::http::Operation,
};

impl crate::application::ApplicationModel {
    pub fn handle_podcast_dashboard_event(
        &mut self,
        event: super::PodcastDashboardEvent,
    ) -> crate::ApplicationCommand {
        let Some(token) = self.session.as_ref().map(|session| session.token.as_str()) else {
            return render();
        };
        let Some(server_url) = self.server_url.as_deref() else {
            return render();
        };
        let ApplicationState::Authenticated(AuthenticatedModel::PodcastDashboard(model)) =
            &mut self.state
        else {
            return render();
        };
        match event {
            super::PodcastDashboardEvent::ListPodcastSubscription(Operation::Request(_)) => {
                model.error = None;
                model.loading = true;
                crate::ApplicationCommand::all([
                    super::execute::execute(server_url, token),
                    render(),
                ])
            }
            super::PodcastDashboardEvent::ListPodcastSubscription(Operation::Success(data)) => {
                model.data = data;
                model.error = None;
                model.loading = false;
                render()
            }
            super::PodcastDashboardEvent::ListPodcastSubscription(Operation::Error(err)) => {
                model.error = Some(err);
                model.loading = false;
                render()
            }
        }
    }
}

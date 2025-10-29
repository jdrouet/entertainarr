use crux_http::command::Http;
use entertainarr_adapter_http::entity::{ApiResource, podcast::PodcastDocument};

use crate::{effect::http::Operation, entity::podcast::Podcast};

pub fn execute(base_url: &str, token: &str) -> crate::ApplicationCommand {
    let url = format!("{base_url}/api/users/me/podcasts");
    Http::get(url)
        .header("Authorization", format!("Bearer {token}"))
        .expect_json::<ApiResource<Vec<PodcastDocument>>>()
        .build()
        .then_send(|res| {
            match res {
                Ok(mut res) => {
                    let body: ApiResource<Vec<PodcastDocument>> = res.take_body().unwrap();
                    let body = Podcast::from_document_list(body);
                    super::PodcastDashboardEvent::ListPodcastSubscription(Operation::Success(body))
                }
                Err(err) => super::PodcastDashboardEvent::ListPodcastSubscription(
                    Operation::Error(err.into()),
                ),
            }
            .into()
        })
}

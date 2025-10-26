pub async fn handle() -> axum::http::StatusCode {
    axum::http::StatusCode::NO_CONTENT
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn should_answer() {
        assert_eq!(super::handle().await.as_u16(), 204);
    }
}

#[cfg(test)]
mod integration {
    use std::sync::Arc;

    use tower::ServiceExt;

    use crate::server::ServerState;
    use crate::server::handler::client::prelude::MockClientService;

    #[tokio::test]
    async fn should_answer() {
        let state = ServerState {
            authentication_service: Arc::new(
                entertainarr_domain::auth::prelude::MockAuthenticationService::new(),
            ),
            client_service: MockClientService::default(),
            podcast_service: Arc::new(
                entertainarr_domain::podcast::prelude::MockPodcastService::new(),
            ),
            podcast_episode_service: Arc::new(
                entertainarr_domain::podcast::prelude::MockPodcastEpisodeService::new(),
            ),
        };
        let router = crate::server::handler::create().with_state(state);
        let res = router
            .oneshot(
                axum::http::Request::builder()
                    .uri("/api")
                    .method(axum::http::Method::HEAD)
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), axum::http::StatusCode::NO_CONTENT);
    }
}

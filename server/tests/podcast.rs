mod shared;

#[tokio::test]
async fn workflow() {
    let server = shared::Server::new().await;
    let mut client = server.client();

    // should create an account
    let token = client
        .auth_signup("user@example.com", "password")
        .await
        .unwrap();
    client.set_token(token);

    // should subscribe podcast
    client
        .podcast_subscribe("https://rustacean-station.org/podcast.rss")
        .await
        .unwrap();

    // should list podcasts
    let payload = client.podcast_episode_list().await.unwrap();
    assert!(payload.data.len() >= 50);
    assert_eq!(payload.includes.len(), 1);
}

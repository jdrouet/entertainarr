use reqwest::StatusCode;

mod shared;

#[tokio::test]
async fn workflow() {
    let client = shared::Client::new().await;

    // should create an account
    let token = client.auth_signup("user@example.com", "password").await;

    // should subscribe podcast
    let res = client
        .client
        .post("http://localhost:3000/api/users/me/podcasts")
        .header("Authorization", format!("Bearer {token}"))
        .json(&serde_json::json!({
            "feedUrl": "https://rustacean-station.org/podcast.rss"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::CREATED);

    // should list podcasts
    let res = client
        .client
        .get("http://localhost:3000/api/podcast-episodes?filter[watched]=false&filter[subscribed]=true")
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let payload: serde_json::Value = res.json().await.unwrap();
    let payload = payload.as_object().unwrap();
    let data = payload.get("data").unwrap().as_array().unwrap();
    assert!(data.len() >= 50);
    let includes = payload.get("includes").unwrap().as_array().unwrap();
    assert_eq!(includes.len(), 1);
}

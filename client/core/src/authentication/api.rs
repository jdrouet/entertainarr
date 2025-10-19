use crux_http::command::Http;

#[derive(serde::Serialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

fn post_auth(
    server_url: &str,
    name: &str,
    payload: &LoginPayload,
) -> crux_core::Command<crate::Effect, crate::Event> {
    let url = format!("{server_url}/api/auth/{name}");
    Http::post(url)
        .body_json(payload)
        .unwrap()
        .expect_json()
        .build()
        .then_send(|res| {
            crate::Event::Authentication(super::AuthenticationEvent::LoginCallback(res))
        })
}

pub fn login(
    server_url: &str,
    payload: &LoginPayload,
) -> crux_core::Command<crate::Effect, crate::Event> {
    post_auth(server_url, "login", payload)
}

pub fn signup(
    server_url: &str,
    payload: &LoginPayload,
) -> crux_core::Command<crate::Effect, crate::Event> {
    post_auth(server_url, "signup", payload)
}

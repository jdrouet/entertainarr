use crux_http::command::Http;

use crate::authentication::AuthenticationKind;

#[derive(serde::Serialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

pub fn execute(
    base_url: &str,
    kind: AuthenticationKind,
    payload: &LoginPayload,
) -> crux_core::Command<crate::Effect, crate::Event> {
    let url = format!("{base_url}/api/auth/{}", kind.as_str());
    tracing::info!("POST {url}");
    Http::post(url)
        .body_json(payload)
        .expect("json body")
        .expect_json()
        .build()
        .then_send(|res| {
            crate::Event::Authentication(super::AuthenticationEvent::Callback(res.into()))
        })
}

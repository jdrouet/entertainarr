use crux_http::command::Http;
use entertainarr_adapter_http::entity::{
    ApiResource,
    auth::{
        AuthenticationRequestAttributes, AuthenticationRequestDocument, AuthenticationTokenDocument,
    },
};

use crate::authentication::AuthenticationKind;

#[derive(serde::Serialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

pub fn execute(
    base_url: &str,
    kind: AuthenticationKind,
    payload: AuthenticationRequestAttributes,
) -> crux_core::Command<crate::Effect, crate::Event> {
    let url = format!("{base_url}/api/auth/{}", kind.as_str());
    tracing::info!("POST {url}");
    Http::post(url)
        .body_json(&ApiResource::new(AuthenticationRequestDocument {
            kind: Default::default(),
            attributes: payload,
        }))
        .expect("json body")
        .expect_json::<ApiResource<AuthenticationTokenDocument>>()
        .build()
        .then_send(|res| match res {
            Ok(mut res) => {
                let payload = res.take_body().unwrap();
                crate::Event::Authentication(super::AuthenticationEvent::Success(payload.data.id))
            }
            Err(err) => crate::Event::Authentication(super::AuthenticationEvent::Error(err)),
        })
}

use crux_http::command::Http;
use entertainarr_adapter_http::entity::{ApiResource, podcast::PodcastSubscribeDocument};

impl super::PodcastSubscribeRequest {
    pub fn execute(self, base_url: &str, token: &str) -> crate::ApplicationCommand {
        let url = format!("{base_url}/api/users/me/podcasts");
        Http::post(url)
            .header("Authorization", format!("Bearer {token}"))
            .body_json(&ApiResource::new(PodcastSubscribeDocument::new(self.url)))
            .expect("json body")
            .build()
            .then_send(|res| {
                match res {
                    Ok(_) => super::PodcastSubscribeEvent::Success,
                    Err(err) => super::PodcastSubscribeEvent::Error(err.into()),
                }
                .into()
            })
    }
}

impl From<crux_http::HttpError> for super::PodcastSubscribeError {
    fn from(err: crux_http::HttpError) -> Self {
        tracing::error!(error = ?err, "unable to subscribe");
        Self::Network
    }
}

#[cfg(test)]
mod tests {
    use crux_http::{HttpError, http::StatusCode};
    use entertainarr_adapter_http::entity::auth::errors::{
        CODE_EMAIL_CONFLICT, CODE_EMAIL_TOO_SHORT, CODE_PASSWORD_TOO_SHORT,
    };

    use crate::application::authentication::AuthenticationError;

    #[test]
    fn should_decode_email_too_short() {
        let err = HttpError::Http {
            code: StatusCode::BadRequest,
            message: String::default(),
            body: Some(
                serde_json::to_vec(&serde_json::json!({
                    "message": "invalid credentials",
                    "detail": {
                        "attribute": "email",
                        "code": CODE_EMAIL_TOO_SHORT,
                    }
                }))
                .unwrap(),
            ),
        };
        let decoded = AuthenticationError::from(err);
        assert_eq!(decoded, AuthenticationError::EmailTooShort);
    }

    #[test]
    fn should_decode_email_conflict() {
        let err = HttpError::Http {
            code: StatusCode::BadRequest,
            message: String::default(),
            body: Some(
                serde_json::to_vec(&serde_json::json!({
                    "message": "invalid credentials",
                    "detail": {
                        "attribute": "email",
                        "code": CODE_EMAIL_CONFLICT,
                    }
                }))
                .unwrap(),
            ),
        };
        let decoded = AuthenticationError::from(err);
        assert_eq!(decoded, AuthenticationError::EmailConflict);
    }

    #[test]
    fn should_decode_password_too_short() {
        let err = HttpError::Http {
            code: StatusCode::BadRequest,
            message: String::default(),
            body: Some(
                serde_json::to_vec(&serde_json::json!({
                    "message": "invalid credentials",
                    "detail": {
                        "attribute": "password",
                        "code": CODE_PASSWORD_TOO_SHORT,
                    }
                }))
                .unwrap(),
            ),
        };
        let decoded = AuthenticationError::from(err);
        assert_eq!(decoded, AuthenticationError::PasswordTooShort);
    }

    #[test]
    fn should_decode_invalid_credentials() {
        let err = HttpError::Http {
            code: StatusCode::BadRequest,
            message: String::default(),
            body: Some(
                serde_json::to_vec(&serde_json::json!({
                    "message": "invalid credentials",
                }))
                .unwrap(),
            ),
        };
        let decoded = AuthenticationError::from(err);
        assert_eq!(decoded, AuthenticationError::InvalidCredentials);
    }

    #[test]
    fn should_decode_network_error_because_no_message() {
        let err = HttpError::Http {
            code: StatusCode::BadRequest,
            message: String::default(),
            body: None,
        };
        let decoded = AuthenticationError::from(err);
        assert_eq!(decoded, AuthenticationError::Network);
    }

    #[test]
    fn should_decode_network_error_from_io_error() {
        let err = HttpError::Io(String::default());
        let decoded = AuthenticationError::from(err);
        assert_eq!(decoded, AuthenticationError::Network);
    }
}

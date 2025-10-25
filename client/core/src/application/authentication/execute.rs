use crux_http::command::Http;
use entertainarr_adapter_http::entity::{
    ApiError, ApiResource,
    auth::{
        AuthenticationRequestAttributes, AuthenticationRequestDocument,
        AuthenticationTokenDocument,
        errors::{CODE_EMAIL_CONFLICT, CODE_EMAIL_TOO_SHORT, CODE_PASSWORD_TOO_SHORT},
    },
};

impl super::AuthenticationKind {
    const fn as_str(&self) -> &'static str {
        match self {
            Self::Login => "login",
            Self::Signup => "signup",
        }
    }
}

impl super::AuthenticationRequest {
    pub fn execute(self, base_url: &str) -> crate::ApplicationCommand {
        let url = format!("{base_url}/api/auth/{}", self.kind.as_str());
        Http::post(url)
            .body_json(&ApiResource::new(AuthenticationRequestDocument {
                kind: Default::default(),
                attributes: AuthenticationRequestAttributes {
                    email: self.email.into(),
                    password: self.password.into(),
                },
            }))
            .expect("json body")
            .expect_json::<ApiResource<AuthenticationTokenDocument>>()
            .build()
            .then_send(|res| {
                match res {
                    Ok(mut res) => {
                        let payload = res.take_body().unwrap();
                        super::AuthenticationEvent::Success(super::AuthenticationSuccess {
                            token: payload.data.id,
                        })
                    }
                    Err(err) => {
                        super::AuthenticationEvent::Error(super::AuthenticationError::from(err))
                    }
                }
                .into()
            })
    }
}

impl From<crux_http::HttpError> for super::AuthenticationError {
    fn from(err: crux_http::HttpError) -> Self {
        match err {
            crux_http::HttpError::Http {
                code: crux_http::http::StatusCode::BadRequest,
                message: _,
                body: Some(body),
            } => match serde_json::from_slice::<'_, ApiError>(body.as_slice()) {
                Ok(body) => {
                    if let Some(detail) = body.detail {
                        if detail.code == CODE_EMAIL_TOO_SHORT {
                            Self::EmailTooShort
                        } else if detail.code == CODE_EMAIL_CONFLICT {
                            Self::EmailConflict
                        } else if detail.code == CODE_PASSWORD_TOO_SHORT {
                            Self::PasswordTooShort
                        } else {
                            Self::InvalidCredentials
                        }
                    } else {
                        Self::InvalidCredentials
                    }
                }
                Err(err) => {
                    tracing::error!(error = ?err, "unable to deserialize error");
                    Self::Network
                }
            },
            err => {
                tracing::error!(error = ?err, "unable to authenticate");
                Self::Network
            }
        }
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

use axum::{Json, extract::State};

use crate::{
    adapter::http_server::handler::{ApiError, ApiErrorDetail},
    domain::auth::{
        entity::{Email, EmailError, Password, PasswordError},
        prelude::{AuthenticationService, SignupError, SignupRequest},
    },
};

#[derive(Debug, serde::Deserialize)]
pub struct SignupPayload {
    email: String,
    password: String,
}

#[derive(Debug, serde::Serialize)]
pub struct SignupResponse {
    token: String,
}

pub async fn handle<S>(
    State(state): State<S>,
    Json(payload): Json<SignupPayload>,
) -> Result<Json<SignupResponse>, ApiError>
where
    S: crate::adapter::http_server::prelude::ServerState,
{
    let email = Email::try_new(payload.email).map_err(|err| {
        let reason = match err {
            EmailError::NotEmptyViolated => "should not be empty",
        };
        ApiError::bad_request("invalid credentials")
            .with_detail(ApiErrorDetail::new("email", reason))
    })?;
    let password = Password::try_new(payload.password).map_err(|err| {
        let reason = match err {
            PasswordError::NotEmptyViolated => "should not be empty",
            PasswordError::LenCharMinViolated => "should be more than 8 characters",
        };
        ApiError::bad_request("invalid credentials")
            .with_detail(ApiErrorDetail::new("password", reason))
    })?;
    state
        .authentication_service()
        .signup(SignupRequest { email, password })
        .await
        .map(|res| Json(SignupResponse { token: res.token }))
        .map_err(|err| match err {
            SignupError::EmailConflict => ApiError::conflict("user conflict")
                .with_detail(ApiErrorDetail::new("email", "already used")),
            SignupError::Internal(err) => {
                tracing::error!(error = %err, error.stacktrace = ?err, "unable to login");
                ApiError::internal()
            }
        })
}

#[cfg(test)]
mod tests {
    use axum::{Json, extract::State, http::StatusCode};

    use crate::{
        adapter::http_server::prelude::tests::MockServerState,
        domain::auth::prelude::{LoginSuccess, MockAuthenticationService},
    };

    #[tokio::test]
    async fn should_succeed() {
        let mut auth_service = MockAuthenticationService::new();
        auth_service.expect_signup().returning(|req| {
            assert_eq!(req.email.into_inner(), "user@example.com");
            assert_eq!(req.password.into_inner(), "password");

            Box::pin(async move {
                Ok(LoginSuccess {
                    token: String::from("token"),
                })
            })
        });
        let state = MockServerState::builder()
            .authentication(auth_service)
            .build();
        let payload = super::SignupPayload {
            email: String::from("user@example.com"),
            password: String::from("password"),
        };
        assert!(super::handle(State(state), Json(payload)).await.is_ok());
    }

    #[tokio::test]
    async fn should_fail_validation_invalid_username() {
        let state = MockServerState::default();
        let payload = super::SignupPayload {
            email: String::from("  "),
            password: String::from("password"),
        };
        let err = super::handle(State(state), Json(payload))
            .await
            .unwrap_err();
        assert_eq!(err.status_code, StatusCode::BAD_REQUEST);
        assert_eq!(err.message, "invalid credentials");
        let detail = err.detail.unwrap();
        assert_eq!(detail.attribute, "email");
        assert_eq!(detail.reason, "should not be empty");
    }

    #[tokio::test]
    async fn should_fail_validation_empty_password() {
        let state = MockServerState::default();
        let payload = super::SignupPayload {
            email: String::from("user@example.com"),
            password: String::from("          "),
        };
        let err = super::handle(State(state), Json(payload))
            .await
            .unwrap_err();
        assert_eq!(err.status_code, StatusCode::BAD_REQUEST);
        assert_eq!(err.message, "invalid credentials");
        let detail = err.detail.unwrap();
        assert_eq!(detail.attribute, "password");
        assert_eq!(detail.reason, "should not be empty");
    }

    #[tokio::test]
    async fn should_fail_validation_invalid_password() {
        let state = MockServerState::default();
        let payload = super::SignupPayload {
            email: String::from("user@example.com"),
            password: String::from("foo"),
        };
        let err = super::handle(State(state), Json(payload))
            .await
            .unwrap_err();
        assert_eq!(err.status_code, StatusCode::BAD_REQUEST);
        assert_eq!(err.message, "invalid credentials");
        let detail = err.detail.unwrap();
        assert_eq!(detail.attribute, "password");
        assert_eq!(detail.reason, "should be more than 8 characters");
    }

    #[tokio::test]
    async fn should_fail_conflicting_email() {
        let mut auth_service = MockAuthenticationService::new();
        auth_service.expect_signup().returning(|req| {
            assert_eq!(req.email.into_inner(), "user@example.com");
            assert_eq!(req.password.into_inner(), "password");

            Box::pin(async move { Err(crate::domain::auth::prelude::SignupError::EmailConflict) })
        });
        let state = MockServerState::builder()
            .authentication(auth_service)
            .build();
        let payload = super::SignupPayload {
            email: String::from("user@example.com"),
            password: String::from("password"),
        };
        let err = super::handle(State(state), Json(payload))
            .await
            .unwrap_err();
        assert_eq!(err.status_code, StatusCode::CONFLICT);
        assert_eq!(err.message, "user conflict");
        let detail = err.detail.unwrap();
        assert_eq!(detail.attribute, "email");
        assert_eq!(detail.reason, "already used");
    }
}

#[cfg(test)]
mod integration {
    use tower::ServiceExt;

    use crate::{
        adapter::http_server::prelude::tests::MockServerState,
        domain::auth::prelude::{LoginSuccess, MockAuthenticationService},
    };

    #[tokio::test]
    async fn should_answer() {
        let router = crate::adapter::http_server::handler::create();
        let mut auth_service = MockAuthenticationService::new();
        auth_service.expect_signup().returning(|req| {
            assert_eq!(req.email.into_inner(), "user@example.com");
            assert_eq!(req.password.into_inner(), "password");

            Box::pin(async move {
                Ok(LoginSuccess {
                    token: String::from("token"),
                })
            })
        });
        let state = MockServerState::builder()
            .authentication(auth_service)
            .build();
        let res = router
            .with_state(state)
            .oneshot(
                axum::http::Request::builder()
                    .uri("/api/auth/signup")
                    .method(axum::http::Method::POST)
                    .header("Content-Type", "application/json")
                    .body(axum::body::Body::from(
                        r#"{"email":"user@example.com","password":"password"}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), axum::http::StatusCode::OK);
    }
}

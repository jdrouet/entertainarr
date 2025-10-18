use axum::{Json, extract::State};
use entertainarr_adapter_http::entity::auth::AuthenticationRequest;

use crate::{
    adapter::http_server::handler::{ApiError, ApiErrorDetail},
    domain::auth::{
        entity::{Email, EmailError, Password, PasswordError},
        prelude::{AuthenticationService, LoginError, LoginRequest},
    },
};

#[derive(Debug, serde::Serialize)]
pub struct LoginResponse {
    token: String,
}

pub async fn handle<S>(
    State(state): State<S>,
    Json(payload): Json<AuthenticationRequest<'static>>,
) -> Result<Json<LoginResponse>, ApiError>
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
        .login(LoginRequest { email, password })
        .await
        .map(|res| Json(LoginResponse { token: res.token }))
        .map_err(|err| match err {
            LoginError::InvalidCredentials => ApiError::bad_request("invalid credentials"),
            LoginError::Internal(err) => {
                tracing::error!(error = %err, error.stacktrace = ?err, "unable to login");
                ApiError::internal()
            }
        })
}

#[cfg(test)]
mod tests {
    use axum::{Json, extract::State, http::StatusCode};
    use entertainarr_adapter_http::entity::auth::AuthenticationRequest;

    use crate::{
        adapter::http_server::prelude::tests::MockServerState,
        domain::auth::prelude::{LoginSuccess, MockAuthenticationService},
    };

    #[tokio::test]
    async fn should_succeed() {
        let mut auth_service = MockAuthenticationService::new();
        auth_service.expect_login().returning(|req| {
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
        let payload = AuthenticationRequest {
            email: "user@example.com".into(),
            password: "password".into(),
        };
        assert!(super::handle(State(state), Json(payload)).await.is_ok());
    }

    #[tokio::test]
    async fn should_fail_validation_invalid_username() {
        let state = MockServerState::default();
        let payload = AuthenticationRequest {
            email: "  ".into(),
            password: "password".into(),
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
        let payload = AuthenticationRequest {
            email: "user@example.com".into(),
            password: "          ".into(),
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
        let payload = AuthenticationRequest {
            email: "user@example.com".into(),
            password: "foo".into(),
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
        auth_service.expect_login().returning(|req| {
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
                    .uri("/api/auth/login")
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

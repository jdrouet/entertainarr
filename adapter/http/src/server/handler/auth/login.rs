use axum::{Json, extract::State};

use entertainarr_domain::auth::{
    entity::{Email, Password},
    prelude::{AuthenticationService, LoginError, LoginRequest},
};

use crate::entity::auth::{
    AuthenticationRequestDocument, AuthenticationTokenDocument,
    errors::{CODE_EMAIL_TOO_SHORT, CODE_PASSWORD_TOO_SHORT},
};
use crate::entity::{ApiError, ApiErrorDetail, ApiResource};

pub async fn handle<S>(
    State(state): State<S>,
    Json(payload): Json<ApiResource<AuthenticationRequestDocument<'static>>>,
) -> Result<Json<ApiResource<AuthenticationTokenDocument>>, ApiError>
where
    S: crate::server::prelude::ServerState,
{
    let email = Email::try_new(payload.data.attributes.email).map_err(|_| {
        ApiError::bad_request("invalid credentials")
            .with_detail(ApiErrorDetail::new("email", CODE_EMAIL_TOO_SHORT))
    })?;
    let password = Password::try_new(payload.data.attributes.password).map_err(|_| {
        ApiError::bad_request("invalid credentials")
            .with_detail(ApiErrorDetail::new("password", CODE_PASSWORD_TOO_SHORT))
    })?;
    state
        .authentication_service()
        .login(LoginRequest { email, password })
        .await
        .map(|res| {
            Json(ApiResource::new(AuthenticationTokenDocument {
                id: res.token,
                kind: Default::default(),
                attributes: Default::default(),
            }))
        })
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
    use crate::entity::auth::AuthenticationRequestDocument;
    use crate::server::prelude::tests::MockServerState;

    use axum::{Json, extract::State, http::StatusCode};
    use entertainarr_domain::auth::prelude::{LoginSuccess, MockAuthenticationService};

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
        let payload = AuthenticationRequestDocument::new("user@example.com", "password");
        assert!(
            super::handle(State(state), Json(crate::entity::ApiResource::new(payload)))
                .await
                .is_ok()
        );
    }

    #[tokio::test]
    async fn should_fail_validation_invalid_username() {
        let state = MockServerState::default();
        let payload = AuthenticationRequestDocument::new("  ", "password");
        let err = super::handle(State(state), Json(crate::entity::ApiResource::new(payload)))
            .await
            .unwrap_err();
        assert_eq!(err.status_code, StatusCode::BAD_REQUEST);
        assert_eq!(err.message, "invalid credentials");
        let detail = err.detail.unwrap();
        assert_eq!(detail.attribute, "email");
        assert_eq!(detail.code, "email-too-short");
    }

    #[tokio::test]
    async fn should_fail_validation_empty_password() {
        let state = MockServerState::default();
        let payload = AuthenticationRequestDocument::new("user@example.com", "          ");
        let err = super::handle(State(state), Json(crate::entity::ApiResource::new(payload)))
            .await
            .unwrap_err();
        assert_eq!(err.status_code, StatusCode::BAD_REQUEST);
        assert_eq!(err.message, "invalid credentials");
        let detail = err.detail.unwrap();
        assert_eq!(detail.attribute, "password");
        assert_eq!(detail.code, "password-too-short");
    }

    #[tokio::test]
    async fn should_fail_validation_invalid_password() {
        let state = MockServerState::default();
        let payload = AuthenticationRequestDocument::new("user@example.com", "foo");
        let err = super::handle(State(state), Json(crate::entity::ApiResource::new(payload)))
            .await
            .unwrap_err();
        assert_eq!(err.status_code, StatusCode::BAD_REQUEST);
        assert_eq!(err.message, "invalid credentials");
        let detail = err.detail.unwrap();
        assert_eq!(detail.attribute, "password");
        assert_eq!(detail.code, "password-too-short");
    }
}

#[cfg(test)]
mod integration {
    use crate::server::prelude::tests::MockServerState;

    use entertainarr_domain::auth::prelude::{LoginSuccess, MockAuthenticationService};
    use tower::ServiceExt;

    #[tokio::test]
    async fn should_answer() {
        let router = crate::server::handler::create();
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
                        r#"{"data":{"attributes":{"email":"user@example.com","password":"password"},"type":"authentication-requests"}}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), axum::http::StatusCode::OK);
    }
}

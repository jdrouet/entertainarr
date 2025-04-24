use axum::{Extension, http::StatusCode};
use entertainarr_database::Database;

use super::error::ApiError;

pub(super) async fn handle(Extension(db): Extension<Database>) -> Result<StatusCode, ApiError> {
    db.ping()
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(ApiError::from)
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;

    #[tokio::test]
    async fn should_ping() {
        let db = entertainarr_database::Config::default()
            .build()
            .await
            .unwrap();
        db.migrate().await.unwrap();
        let code = super::handle(axum::Extension(db)).await.unwrap();
        assert_eq!(code, StatusCode::NO_CONTENT);
    }
}

use axum::Extension;
use entertainarr_web::login::LoginView;

use crate::handler::view::error::ViewError;
use crate::service::database::Database;

impl entertainarr_web::login::User for crate::model::user::Entity {
    fn login_url(&self) -> String {
        format!("/?user={}", self.id)
    }

    fn name(&self) -> &str {
        &self.name
    }
}

pub(super) async fn handle(
    Extension(database): Extension<Database>,
) -> Result<super::View<LoginView<crate::model::user::Entity>>, ViewError> {
    let list = crate::model::user::list(database.as_ref()).await?;
    Ok(super::View(LoginView::new(list)))
}

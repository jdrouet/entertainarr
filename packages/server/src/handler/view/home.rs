use entertainarr_web::home::HomeView;

use crate::handler::view::error::ViewError;

pub(super) async fn handle() -> Result<super::View, ViewError> {
    Ok(super::View::from(HomeView {}))
}

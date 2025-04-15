use entertainarr_web::home::HomeView;

use crate::handler::view::error::ViewError;

pub(super) async fn handle() -> Result<super::View<HomeView>, ViewError> {
    Ok(super::View(HomeView {}))
}

pub mod authentication;
pub mod home;
pub mod init;

pub enum AuthenticatedModel {
    Home(home::HomeModel),
}

impl Default for AuthenticatedModel {
    fn default() -> Self {
        Self::Home(Default::default())
    }
}

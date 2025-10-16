use entertainarr_client_core::Event;

pub mod authentication;
pub mod home;
pub mod init;

pub trait Render {
    fn render(&self) -> inquire::error::InquireResult<Event>;
}

impl Render for entertainarr_client_core::View {
    fn render(&self) -> inquire::error::InquireResult<Event> {
        match self {
            entertainarr_client_core::View::Authentication(inner) => inner.render(),
            entertainarr_client_core::View::Init(inner) => inner.render(),
            entertainarr_client_core::View::Home(inner) => inner.render(),
        }
    }
}

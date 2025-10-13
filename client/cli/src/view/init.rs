use entertainarr_client_core::Event;
use inquire::Text;

impl super::Render for entertainarr_client_core::init::View {
    fn render(&self) -> inquire::error::InquireResult<Event> {
        let mut server_url =
            Text::new("Server URL:").with_validator(inquire::required!("This field is required"));
        server_url.default = self.server_url.as_deref();
        let server_url = server_url.prompt()?;

        Ok(Event::Init(entertainarr_client_core::init::Event {
            server_url,
        }))
    }
}

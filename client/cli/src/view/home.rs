use entertainarr_client_core::Event;

impl super::Render for entertainarr_client_core::home::HomeView {
    fn render(&self) -> inquire::error::InquireResult<Event> {
        std::thread::sleep(std::time::Duration::from_secs(1));
        Ok(Event::Noop)
    }
}

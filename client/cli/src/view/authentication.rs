use entertainarr_client_core::{Event, authentication::AuthenticationKind};
use inquire::{Password, Select, Text};

impl super::Render for entertainarr_client_core::authentication::AuthenticationView {
    fn render(&self) -> inquire::error::InquireResult<Event> {
        let email = Text::new("Email:")
            .with_validator(inquire::required!("This field is required"))
            .prompt()?;
        let password = Password::new("Password:")
            .with_validator(inquire::required!("This field is required"))
            .with_validator(inquire::min_length!(8))
            .without_confirmation()
            .prompt()?;
        let options = vec!["Login", "Signup"];
        let kind = Select::new("Authentication", options).prompt()?;

        let kind = if kind == "Login" {
            AuthenticationKind::Login
        } else {
            AuthenticationKind::Signup
        };

        Ok(Event::Authentication(
            entertainarr_client_core::authentication::AuthenticationEvent::Execute {
                email,
                password,
                kind,
            },
        ))
    }
}

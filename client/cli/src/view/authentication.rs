use entertainarr_client_core::Event;
use inquire::{Password, Select, Text};

impl super::Render for entertainarr_client_core::authentication::View {
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

        if kind == "Login" {
            Ok(Event::Authentication(
                entertainarr_client_core::authentication::Event::Login { email, password },
            ))
        } else {
            Ok(Event::Authentication(
                entertainarr_client_core::authentication::Event::Signup { email, password },
            ))
        }
    }
}

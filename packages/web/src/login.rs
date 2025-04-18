use askama::Template;

pub trait User {
    fn login_url(&self) -> String;
    fn name(&self) -> &str;
}

#[derive(Debug, Default, Template)]
#[template(path = "view/login.html")]
pub struct LoginView {
    error: Option<&'static str>,
}

impl LoginView {
    pub fn new(error: Option<&'static str>) -> Self {
        Self { error }
    }
}

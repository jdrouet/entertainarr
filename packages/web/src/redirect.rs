use std::borrow::Cow;

use askama::Template;

#[derive(Debug, Template)]
#[template(path = "view/redirect.html")]
pub struct RedirectView {
    redirect_url: Cow<'static, str>,
}

impl RedirectView {
    pub fn new(redirect_url: impl Into<Cow<'static, str>>) -> Self {
        Self {
            redirect_url: redirect_url.into(),
        }
    }
}

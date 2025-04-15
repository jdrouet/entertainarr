use askama::Template;

#[derive(Debug, Template)]
#[template(path = "view/home.html")]
pub struct HomeView {}

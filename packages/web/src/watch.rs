use askama::Template;

#[derive(Debug)]
pub struct Source {
    kind: String,
    src: String,
}

impl Source {
    pub fn new(kind: String, src: String) -> Self {
        Self { kind, src }
    }
}

#[derive(Debug, Template)]
#[template(path = "view/watch.html")]
pub struct WatchView {
    sources: Vec<Source>,
}

impl WatchView {
    pub fn new(sources: Vec<Source>) -> Self {
        Self { sources }
    }
}

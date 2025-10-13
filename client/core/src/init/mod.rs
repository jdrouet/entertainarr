#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct Event {
    pub server_url: String,
}

#[derive(Debug, Default)]
pub struct View {
    pub server_url: Option<String>,
}

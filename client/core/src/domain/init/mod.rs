mod update;

#[derive(Clone, Debug, Eq, PartialEq, facet::Facet, serde::Serialize, serde::Deserialize)]
pub struct Event {
    pub server_url: String,
    pub authentication_token: Option<String>,
}

impl Event {
    pub fn name(&self) -> &'static str {
        "init"
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct View;

mod update;

#[derive(Clone, Debug, Eq, PartialEq, facet::Facet, serde::Serialize, serde::Deserialize)]
pub struct InitEvent {
    pub server_url: String,
    pub authentication_token: Option<String>,
}

impl InitEvent {
    pub fn name(&self) -> &'static str {
        "init"
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct InitView;

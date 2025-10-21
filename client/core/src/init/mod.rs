#[derive(Clone, Debug, Eq, PartialEq, facet::Facet, serde::Serialize, serde::Deserialize)]
pub struct InitEvent {
    pub server_url: String,
    pub authentication_token: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct InitView {
    pub server_url: Option<String>,
}

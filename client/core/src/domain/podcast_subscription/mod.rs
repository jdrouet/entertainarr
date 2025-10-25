mod update;

#[derive(Default)]
pub struct Model {
    pub loading: bool,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct View {
    pub loading: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, facet::Facet, serde::Serialize, serde::Deserialize)]
#[repr(C)]
pub enum Event {
    Open,
    Submit { url: String },
    Success,
    Error(crux_http::HttpError),
}

impl Event {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Open => "podcast_subscription.open",
            Self::Submit { .. } => "podcast_subscription.submit",
            Self::Success => "podcast_subscription.success",
            Self::Error(_) => "podcast_subscription.error",
        }
    }
}

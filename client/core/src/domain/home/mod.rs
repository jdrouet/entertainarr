mod update;

#[derive(Default)]
pub struct HomeModel {
    pub podcasts_loading: bool,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct HomeView {
    pub podcasts_loading: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, facet::Facet, serde::Serialize, serde::Deserialize)]
#[repr(C)]
pub enum HomeEvent {
    Initialize,
}

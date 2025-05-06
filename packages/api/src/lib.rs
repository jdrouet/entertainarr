pub mod tvshow;
pub mod tvshow_episode;
pub mod tvshow_season;
pub mod user;

fn is_u16_zero(value: &u16) -> bool {
    *value == 0
}

fn is_u32_zero(value: &u32) -> bool {
    *value == 0
}

fn is_u64_zero(value: &u64) -> bool {
    *value == 0
}

fn is_false(value: &bool) -> bool {
    !*value
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Response<V, R = (), M = ()>
where
    R: IsEmpty,
    M: IsEmpty,
{
    pub data: V,
    #[serde(skip_serializing_if = "IsEmpty::is_empty")]
    pub relationships: R,
    #[serde(skip_serializing_if = "IsEmpty::is_empty")]
    pub meta: M,
}

pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

impl IsEmpty for () {
    fn is_empty(&self) -> bool {
        true
    }
}

impl<V> IsEmpty for Option<V> {
    fn is_empty(&self) -> bool {
        self.is_none()
    }
}

impl<V> IsEmpty for Vec<V> {
    fn is_empty(&self) -> bool {
        Vec::is_empty(self)
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MetaCount {
    pub count: u32,
}

impl IsEmpty for MetaCount {
    fn is_empty(&self) -> bool {
        false
    }
}

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

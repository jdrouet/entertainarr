use crux_core::{macros::effect, render::RenderOperation};
use crux_http::protocol::HttpRequest;

pub mod http;
pub mod persistence;

#[effect(typegen)]
#[derive(Debug)]
pub enum Effect {
    Http(HttpRequest),
    Persistence(self::persistence::Persistence),
    Render(RenderOperation),
}

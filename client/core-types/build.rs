use crux_core::typegen::TypeGen;
use entertainarr_client_core::Application;
use entertainarr_client_core::application::authenticated::home::HomeEvent;
use entertainarr_client_core::application::authenticated::podcast::subscribe::{
    PodcastSubscribeError, PodcastSubscribeEvent,
};
use entertainarr_client_core::application::authentication::{
    AuthenticationError, AuthenticationEvent, AuthenticationKind,
};
use entertainarr_client_core::application::router::Route;
use entertainarr_client_core::application::{ApplicationView, ApplicationViewModel};
use entertainarr_client_core::entity::podcast_episode::PodcastEpisode;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=../core");

    let mut tgen = TypeGen::new();

    tgen.register_app::<Application>()?;
    tgen.register_type::<PodcastEpisode>()?;
    tgen.register_type::<AuthenticationEvent>()?;
    tgen.register_type::<AuthenticationError>()?;
    tgen.register_type::<AuthenticationKind>()?;
    tgen.register_type::<PodcastSubscribeEvent>()?;
    tgen.register_type::<PodcastSubscribeError>()?;
    tgen.register_type::<Route>()?;
    tgen.register_type::<HomeEvent>()?;
    tgen.register_type::<ApplicationView>()?;
    tgen.register_type::<ApplicationViewModel>()?;

    let output_root = PathBuf::from("./generated");

    // tgen.swift("SharedTypes", output_root.join("swift"))?;

    // tgen.java("com.crux.example.simple_counter", output_root.join("java"))?;

    tgen.typescript("core_types", output_root.join("typescript"))?;

    Ok(())
}

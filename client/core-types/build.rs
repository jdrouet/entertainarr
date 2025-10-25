use crux_core::typegen::TypeGen;
use entertainarr_client_core::domain::authentication::Event;
use entertainarr_client_core::{Application, View};
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=../core");

    let mut tgen = TypeGen::new();

    tgen.register_app::<Application>()?;
    tgen.register_type::<Event>()?;
    tgen.register_type::<View>()?;

    let output_root = PathBuf::from("./generated");

    // tgen.swift("SharedTypes", output_root.join("swift"))?;

    // tgen.java("com.crux.example.simple_counter", output_root.join("java"))?;

    tgen.typescript("core_types", output_root.join("typescript"))?;

    Ok(())
}

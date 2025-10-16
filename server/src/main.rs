#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let tracing = entertainarr::tracing::Config::from_env()?;
    tracing.install()?;

    let path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "./config.toml".into());
    let config = entertainarr::Config::from_path(path)?;
    let app = config.build().await?;
    app.run().await
}

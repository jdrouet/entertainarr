#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let tracing = entertainarr::tracing::Config::from_env()?;
    tracing.install()?;

    let config = entertainarr::Config::from_env()?;
    let app = config.build().await?;
    app.run().await
}

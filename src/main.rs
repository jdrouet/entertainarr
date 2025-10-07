#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let config = entertainarr::Config::from_env()?;
    let app = config.build().await?;
    app.run().await
}

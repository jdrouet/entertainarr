#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = entertainarr::Config::from_env()?;
    let app = config.build()?;
    app.run().await
}

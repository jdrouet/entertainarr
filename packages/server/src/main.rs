use std::path::PathBuf;

use entertainarr_server::Config;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "entertainarr_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config_path = std::env::var("CONFIG_PATH").map(PathBuf::from).ok();
    let config = Config::parse(config_path)?;
    let server = config.build().await?;
    server.prepare().await?;
    server.preload(&config.dataset).await?;
    server.listen().await?;
    Ok(())
}

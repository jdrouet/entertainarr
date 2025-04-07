use entertainarr_server::Config;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = Config::default();
    let server = config.build();
    server.listen().await
}

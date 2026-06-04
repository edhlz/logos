use network::listener::run_listener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();

    run_listener("0.0.0.0:25565").await
}
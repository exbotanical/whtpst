use std::net::TcpListener;
use whtpst::{
    config::get_config,
    dao::{InMemoryRepository, Repository},
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("whtpst".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_config().expect("Failed to read config file");
    let repo = InMemoryRepository::new();

    let address = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(address)?;

    run(listener, repo)?.await?;
    Ok(())
}

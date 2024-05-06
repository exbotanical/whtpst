use gossip::{
    config::get_config,
    dao::{InMemoryRepository, Repository},
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("gossip".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_config().expect("Failed to read config file");
    let repo = InMemoryRepository::new();
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(config.database.with_db());

    let address = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(address)?;

    run(listener, connection_pool, repo)?.await?;
    Ok(())
}

use clap::Parser;
use config_service::{config::Config, router, state::ServiceState};
use lambda_http::{Error, run};
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .json()
        .with_ansi(false)
        .init();

    let config = Config::parse();
    let state = ServiceState {
        webhook_secret: config.webhook_secret,
    };

    let app = router(state);

    info!("Service initialized");

    run(app).await
}

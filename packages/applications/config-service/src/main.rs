use clap::Parser;
use config_service::{config::Config, router, state::ServiceState};
use lambda_http::{Error, run};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().json().with_ansi(false).init();

    let config = Config::parse();

    let state = ServiceState::load(&config).await?;

    let app = router(state);

    info!("Service initialized");

    run(app).await
}

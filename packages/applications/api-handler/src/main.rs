use api_handler::{config::Config, router, state::ServiceState};
use clap::Parser;
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

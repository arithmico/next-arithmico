use clap::Parser;
use config_service::{config::Config, router, state::ServiceState};
use lambda_http::{Error, run};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().json().with_ansi(false).init();

    let config = Config::parse();

    let sdk_config = aws_config::load_from_env().await;
    let sm_client = aws_sdk_secretsmanager::Client::new(&sdk_config);
    let response = sm_client
        .get_secret_value()
        .secret_id(&config.webhook_secret_arn)
        .send()
        .await?;

    let webhook_secret = response
        .secret_string()
        .ok_or_else(|| Error::from("Webhook secret is empty"))?;

    let state = ServiceState {
        webhook_secret: webhook_secret.to_string(),
    };

    let app = router(state);

    info!("Service initialized");

    run(app).await
}

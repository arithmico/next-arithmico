use clap::Parser;
use lambda_runtime::{Error, LambdaEvent, run, service_fn};
use tracing::info;
mod handler;

use crate::{config::Config, handler::Handler};

mod config;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().with_ansi(false).init();

    let config = Config::parse();

    let handler = Handler::init(config).await?;

    info!("Handler initialized");

    run(service_fn(|_event: LambdaEvent<()>| {
        let handler = handler.clone();
        async move { handler.execute().await.map_err(Error::from) }
    }))
    .await
}

use clap::Parser;
use lambda_runtime::{run, service_fn, Error};
mod handler;
use handler::handler;

use crate::config::Config;

mod config;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().with_ansi(false).init();

    let _config = Config::parse();

    run(service_fn(handler)).await
}

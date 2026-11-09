use lambda_runtime::{Error, run, service_fn};
mod handler;
use handler::handler;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().with_ansi(false).init();

    run(service_fn(handler)).await
}

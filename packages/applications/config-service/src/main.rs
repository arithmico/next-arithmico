use config_service::router;
use lambda_http::{Error, run};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = router();
    run(app).await
}

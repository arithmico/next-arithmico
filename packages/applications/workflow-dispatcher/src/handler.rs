use lambda_runtime::{Error, LambdaEvent};

pub(crate) async fn handler(_event: LambdaEvent<()>) -> Result<(), Error> {
    Ok(())
}

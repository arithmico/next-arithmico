use clap::Parser;

#[derive(Debug, Parser)]
pub struct Config {
    #[arg(long, env)]
    pub dynamodb_table_name: String,

    #[arg(long, env)]
    pub github_app_secret_arn: String,

    #[arg(long, env)]
    pub workflow_queue_url: String,
}

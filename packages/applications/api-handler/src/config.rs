use clap::Parser;

#[derive(Debug, Parser)]
pub struct Config {
    #[arg(long, env)]
    pub dynamodb_table_name: String,

    #[arg(long, env)]
    pub github_oidc_configuration_uri: String,

    #[arg(long, env)]
    pub github_app_secret_arn: String,
}

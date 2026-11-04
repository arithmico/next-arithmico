use clap::{Args, Parser};

#[derive(Debug, Parser)]
pub enum Command {
    Verify,
    Show,
    Report(Report),
}

#[derive(Debug, Args)]
pub struct Report {
    #[arg(long, env)]
    pub actions_id_token_request_token: String,

    #[arg(long, env)]
    pub actions_id_token_request_url: String,

    #[arg(long, env)]
    pub config_service_report_url: String,
}

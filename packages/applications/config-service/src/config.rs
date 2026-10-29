use clap::Parser;

#[derive(Debug, Parser)]
pub struct Config {
    #[arg(long, env)]
    pub webhook_secret: String,
}

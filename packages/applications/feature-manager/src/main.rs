use std::process::ExitCode;

use clap::Parser;

use crate::verify::verify;

mod config;
mod feature;
mod verify;

fn main() -> ExitCode {
    let command = config::Command::parse();

    match command {
        config::Command::Verify => {
            if let Err(err) = verify() {
                println!("Error: {}", err);
                ExitCode::FAILURE
            } else {
                ExitCode::SUCCESS
            }
        }
    }
}

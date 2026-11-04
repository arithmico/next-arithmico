use std::process::ExitCode;

use clap::Parser;

use crate::{show::normalize, verify::verify};

mod config;
mod feature;
mod show;
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
        config::Command::Show => match normalize() {
            Ok(output) => {
                println!("{:#?}", output);
                ExitCode::SUCCESS
            }
            Err(err) => {
                println!("Error: {}", err);
                ExitCode::FAILURE
            }
        },
    }
}

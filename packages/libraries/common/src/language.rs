use std::str::FromStr;

use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Hash,
    Eq,
    Serialize,
    Deserialize,
    EnumIter,
    Default,
)]
pub enum Language {
    German,
    #[default]
    English,
}

impl FromStr for Language {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "German" => Ok(Language::German),
            "English" => Ok(Language::English),
            _ => Err("failed to parse language"),
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::German => f.write_str("German"),
            Language::English => f.write_str("English"),
        }
    }
}

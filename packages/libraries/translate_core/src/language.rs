use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub enum Language {
    German,
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

impl ToString for Language {
    fn to_string(&self) -> String {
        match self {
            Language::German => "German".to_string(),
            Language::English => "English".to_string(),
        }
    }
}

use std::fmt::Debug;

use engine::Language;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct OverrideDecimalFormat(Option<Language>);

impl OverrideDecimalFormat {
    pub fn new() -> Self {
        Self(None)
    }

    pub fn decimal_format(&self) -> Option<Language> {
        self.0
    }
}

impl From<Language> for OverrideDecimalFormat {
    fn from(value: Language) -> Self {
        Self(Some(value))
    }
}

impl Debug for OverrideDecimalFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(value) => value.fmt(f),
            None => write!(f, "Default"),
        }
    }
}

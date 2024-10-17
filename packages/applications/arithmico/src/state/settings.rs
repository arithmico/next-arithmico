use std::fmt::Debug;

use common::{DecimalFormat, DecimalPlaces, Language};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub decimal_places: DecimalPlaces,
    pub language: Language,
    pub override_decimal_format: OverrideDecimalFormat,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct OverrideDecimalFormat(Option<DecimalFormat>);

impl OverrideDecimalFormat {
    pub fn new() -> Self {
        Self(None)
    }
}

impl From<DecimalFormat> for OverrideDecimalFormat {
    fn from(value: DecimalFormat) -> Self {
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

impl Default for Settings {
    fn default() -> Self {
        Self {
            decimal_places: DecimalPlaces::from(5),
            language: Language::English,
            override_decimal_format: OverrideDecimalFormat(None),
        }
    }
}

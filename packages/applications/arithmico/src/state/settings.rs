use std::fmt::Debug;

use common::{DecimalPlaces, Language};
use override_decimal_format::OverrideDecimalFormat;
use serde::{Deserialize, Serialize};
use theme::Theme;

pub mod override_decimal_format;
pub mod theme;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub decimal_places: DecimalPlaces,
    pub language: Language,
    pub override_decimal_format: OverrideDecimalFormat,
    pub theme: Theme,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            decimal_places: DecimalPlaces::from(5),
            language: Language::English,
            override_decimal_format: OverrideDecimalFormat::new(),
            theme: Theme::Light,
        }
    }
}

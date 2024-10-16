use common::{DecimalFormat, DecimalPlaces, Language};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub decimal_places: DecimalPlaces,
    pub language: Language,
    pub override_decimal_format: Option<DecimalFormat>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            decimal_places: DecimalPlaces::from(5),
            language: Language::English,
            override_decimal_format: None,
        }
    }
}

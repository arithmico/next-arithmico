use crate::Language;

use super::DecimalPlaces;

#[derive(Debug, Clone, PartialEq)]
pub struct Settings {
    decimal_places: DecimalPlaces,
    language: Language,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            decimal_places: 5.into(),
            language: Language::English,
        }
    }
}

impl Settings {
    pub fn new(decimal_places: u8, language: Language) -> Self {
        Self {
            decimal_places: decimal_places.into(),
            language,
        }
    }

    pub fn get_decimal_places(&self) -> u8 {
        u8::from(&self.decimal_places)
    }

    pub fn get_language(&self) -> Language {
        self.language.clone()
    }
}

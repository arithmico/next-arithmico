use language::Language;

use crate::DecimalPlaces;

#[derive(Debug, Clone, Copy)]
pub struct Options {
    pub language: Language,
    pub decimal_places: DecimalPlaces,
}

impl Options {
    pub fn new(language: Language, decimal_places: DecimalPlaces) -> Self {
        Self {
            language,
            decimal_places,
        }
    }
}

impl Default for Options {
    fn default() -> Self {
        Self {
            language: Default::default(),
            decimal_places: Default::default(),
        }
    }
}

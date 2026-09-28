use common::Language;

use crate::DecimalPlaces;

#[derive(Debug, Clone, Copy)]
#[derive(Default)]
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


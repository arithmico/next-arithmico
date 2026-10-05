use common::{Language, NumberRepresentation};

use crate::DecimalPlaces;

#[derive(Debug, Clone, Copy, Default)]
pub struct Options {
    pub language: Language,
    pub decimal_places: DecimalPlaces,
    pub number_representation: NumberRepresentation,
}

impl Options {
    pub fn new(
        language: Language,
        decimal_places: DecimalPlaces,
        number_representation: NumberRepresentation,
    ) -> Self {
        Self {
            language,
            decimal_places,
            number_representation,
        }
    }
}

use super::DecimalPlaces;

#[derive(Debug, Clone, PartialEq)]
pub struct Settings {
    decimal_places: DecimalPlaces,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            decimal_places: 5.into(),
        }
    }
}

impl Settings {
    pub fn new(decimal_places: u8) -> Self {
        Self {
            decimal_places: decimal_places.into(),
        }
    }

    pub fn get_decimal_places(&self) -> u8 {
        u8::from(&self.decimal_places)
    }
}

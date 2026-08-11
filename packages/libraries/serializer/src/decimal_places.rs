use serde::{Deserialize, Serialize};

const MAX_DECIMAL_PLACES: u8 = 15;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DecimalPlaces(u8);

impl DecimalPlaces {
    pub fn new(value: u8) -> Self {
        Self(value.min(MAX_DECIMAL_PLACES))
    }

    pub fn max() -> u8 {
        MAX_DECIMAL_PLACES
    }
}

impl From<DecimalPlaces> for u8 {
    fn from(value: DecimalPlaces) -> Self {
        value.0
    }
}

impl From<DecimalPlaces> for usize {
    fn from(value: DecimalPlaces) -> Self {
        value.0 as usize
    }
}

impl Default for DecimalPlaces {
    fn default() -> Self {
        Self(5)
    }
}

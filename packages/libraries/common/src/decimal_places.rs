use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecimalPlaces(u8);

impl From<u8> for DecimalPlaces {
    fn from(value: u8) -> Self {
        if value > 15 {
            panic!("invalid decimal places");
        }
        DecimalPlaces(value)
    }
}

impl From<&DecimalPlaces> for u8 {
    fn from(value: &DecimalPlaces) -> Self {
        value.0
    }
}

use crate::core::{DecimalFormat, DecimalPlaces};

pub struct SerializeNodeOptions {
    pub decimal_places: DecimalPlaces,
    pub decimal_format: DecimalFormat,
}

impl Default for SerializeNodeOptions {
    fn default() -> Self {
        Self {
            decimal_places: DecimalPlaces::from(5),
            decimal_format: DecimalFormat::Dot,
        }
    }
}

impl SerializeNodeOptions {
    pub fn new(
        decimal_places: DecimalPlaces,
        decimal_format: DecimalFormat,
    ) -> Self {
        Self {
            decimal_format,
            decimal_places,
        }
    }
}

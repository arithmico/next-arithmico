use common::{DecimalFormat, DecimalPlaces};

#[derive(Debug, Clone, PartialEq)]
pub struct EvaluateNodeOptions {
    decimal_places: DecimalPlaces,
    decimal_format: DecimalFormat,
}

impl Default for EvaluateNodeOptions {
    fn default() -> Self {
        Self {
            decimal_places: 5.into(),
            decimal_format: DecimalFormat::Dot,
        }
    }
}

impl EvaluateNodeOptions {
    pub fn new(decimal_places: u8, decimal_format: DecimalFormat) -> Self {
        Self {
            decimal_places: decimal_places.into(),
            decimal_format,
        }
    }

    pub fn get_decimal_places(&self) -> u8 {
        u8::from(&self.decimal_places)
    }

    pub fn get_decimal_format(&self) -> DecimalFormat {
        self.decimal_format.clone()
    }
}

use common::{DecimalFormat, DecimalPlaces};

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

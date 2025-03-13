use crate::DecimalFormat;

pub struct ParseNodeOptions {
    pub decimal_format: DecimalFormat,
}

impl ParseNodeOptions {
    pub fn new(decimal_format: DecimalFormat) -> Self {
        Self { decimal_format }
    }
}

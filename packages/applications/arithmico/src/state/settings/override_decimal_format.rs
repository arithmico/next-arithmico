use std::fmt::Debug;

use engine::DecimalFormat;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct OverrideDecimalFormat(Option<DecimalFormat>);

impl OverrideDecimalFormat {
    pub fn new() -> Self {
        Self(None)
    }

    pub fn decimal_format(&self) -> Option<&DecimalFormat> {
        self.0.as_ref()
    }
}

impl From<DecimalFormat> for OverrideDecimalFormat {
    fn from(value: DecimalFormat) -> Self {
        Self(Some(value))
    }
}

impl Debug for OverrideDecimalFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(value) => value.fmt(f),
            None => write!(f, "Default"),
        }
    }
}

use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, EnumIter)]
pub enum AngleUnit {
    Radian,
    Degree,
}

impl Default for AngleUnit {
    fn default() -> Self {
        Self::Radian
    }
}

impl AngleUnit {
    pub fn to_radians(&self, value: f64) -> f64 {
        match self {
            AngleUnit::Radian => value,
            AngleUnit::Degree => value.to_radians(),
        }
    }

    pub fn from_radians(&self, value: f64) -> f64 {
        match self {
            AngleUnit::Radian => value,
            AngleUnit::Degree => value.to_degrees(),
        }
    }
}

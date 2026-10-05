use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub enum NumberRepresentation {
    #[default]
    Number,
    Fraction,
    MixedFraction,
}

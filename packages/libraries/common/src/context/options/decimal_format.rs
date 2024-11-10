use serde::{Deserialize, Serialize};
use translate_core::Language;

#[derive(Debug, Clone, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub enum DecimalFormat {
    Comma,
    Dot,
}

impl From<&Language> for DecimalFormat {
    fn from(value: &Language) -> Self {
        match value {
            Language::German => DecimalFormat::Comma,
            Language::English => DecimalFormat::Dot,
        }
    }
}

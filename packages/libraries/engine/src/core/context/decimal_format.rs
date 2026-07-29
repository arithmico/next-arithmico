use serde::{Deserialize, Serialize};
use translate_core::Language;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub enum DecimalFormat {
    Comma,
    Dot,
}

impl From<Language> for DecimalFormat {
    fn from(value: Language) -> Self {
        match value {
            Language::German => DecimalFormat::Comma,
            Language::English => DecimalFormat::Dot,
        }
    }
}

impl From<DecimalFormat> for Language {
    fn from(value: DecimalFormat) -> Self {
        match value {
            DecimalFormat::Comma => Language::German,
            DecimalFormat::Dot => Language::English,
        }
    }
}

impl Default for DecimalFormat {
    fn default() -> Self {
        DecimalFormat::Dot
    }
}

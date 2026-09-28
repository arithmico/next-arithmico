use std::fmt::{Display, Write};

use common::Language;

#[derive(Debug, Clone, Copy)]
pub(crate) enum DecimalSeperator {
    Comma,
    Dot,
}

impl From<Language> for DecimalSeperator {
    fn from(value: Language) -> Self {
        match value {
            Language::German => Self::Comma,
            Language::English => Self::Dot,
        }
    }
}

impl Display for DecimalSeperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecimalSeperator::Comma => f.write_char(','),
            DecimalSeperator::Dot => f.write_char('.'),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum ArgumentSeperator {
    Semicolon,
    Comma,
}

impl From<Language> for ArgumentSeperator {
    fn from(value: Language) -> Self {
        match value {
            Language::German => Self::Semicolon,
            Language::English => Self::Comma,
        }
    }
}

impl Display for ArgumentSeperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgumentSeperator::Semicolon => f.write_char(';'),
            ArgumentSeperator::Comma => f.write_char(','),
        }
    }
}

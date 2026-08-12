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

impl ToString for DecimalSeperator {
    fn to_string(&self) -> String {
        match self {
            DecimalSeperator::Comma => String::from(","),
            DecimalSeperator::Dot => String::from("."),
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

impl ToString for ArgumentSeperator {
    fn to_string(&self) -> String {
        match self {
            ArgumentSeperator::Semicolon => String::from(";"),
            ArgumentSeperator::Comma => String::from(","),
        }
    }
}

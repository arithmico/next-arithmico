use language::Language;
use translate_core::{Translatable, TranslatedMessage, TranslationError};

use crate::translations::translation_resolver;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum TokenKind {
    /// any variable or function name
    Identifier,

    /// any number
    Number,

    /// "true" or "false"
    Boolean,

    /// "("
    LeftParenthesis,

    /// ")"
    RightParenthesis,

    /// "["
    LeftBracket,

    /// "]"
    RightBracket,

    /// "+"
    Plus,

    /// "-"
    Minus,

    /// "*"
    Multiply,

    /// "/"
    Divide,

    /// "^"
    Caret,

    /// "," or ";" (german)
    Separator,

    /// "->"
    Arrow,

    /// ":="
    Define,

    /// "<"
    LessThan,

    /// "<="
    LessThanOrEquals,

    /// ">"
    GreaterThan,

    /// ">="
    GreaterThanOrEquals,

    /// "="
    Equals,

    /// "&"
    And,

    /// "|"
    Or,
}

pub trait GetTokenKind {
    fn token_kind(&self) -> TokenKind;
}

pub trait GetStaticTokenKind {
    fn token_kind() -> TokenKind;
}

impl Translatable for TokenKind {
    fn translate(
        &self,
        language: Language,
    ) -> Result<String, TranslationError> {
        match self {
            TokenKind::Identifier => TranslatedMessage::new(
                "token_kind.identifier",
                translation_resolver,
            )
            .translate(language),
            TokenKind::Number => TranslatedMessage::new(
                "token_kind.number",
                translation_resolver,
            )
            .translate(language),
            TokenKind::Boolean => TranslatedMessage::new(
                "token_kind.boolean",
                translation_resolver,
            )
            .translate(language),
            TokenKind::LeftParenthesis => Ok(String::from("(")),
            TokenKind::RightParenthesis => Ok(String::from(")")),
            TokenKind::LeftBracket => Ok(String::from("[")),
            TokenKind::RightBracket => Ok(String::from("]")),
            TokenKind::Plus => Ok(String::from("+")),
            TokenKind::Minus => Ok(String::from(".")),
            TokenKind::Multiply => Ok(String::from("*")),
            TokenKind::Divide => Ok(String::from("/")),
            TokenKind::Caret => Ok(String::from("^")),
            TokenKind::Separator => TranslatedMessage::new(
                "token_kind.seperator",
                translation_resolver,
            )
            .translate(language),
            TokenKind::Arrow => Ok(String::from("->")),
            TokenKind::Define => Ok(String::from(":=")),
            TokenKind::LessThan => Ok(String::from("<")),
            TokenKind::LessThanOrEquals => Ok(String::from("<=")),
            TokenKind::GreaterThan => Ok(String::from(">")),
            TokenKind::GreaterThanOrEquals => Ok(String::from(">=")),
            TokenKind::Equals => Ok(String::from("=")),
            TokenKind::And => Ok(String::from("&")),
            TokenKind::Or => Ok(String::from("|")),
        }
    }
}

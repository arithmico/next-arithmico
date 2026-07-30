use language::Language;
use translate_core::{
    Quoted, Translatable, TranslatedMessage, TranslationError,
};

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
    Asterisk,

    /// "/"
    Slash,

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
            TokenKind::LeftParenthesis => {
                Ok(String::from("(").quoted(language)?)
            }
            TokenKind::RightParenthesis => {
                Ok(String::from(")").quoted(language)?)
            }
            TokenKind::LeftBracket => Ok(String::from("[").quoted(language)?),
            TokenKind::RightBracket => Ok(String::from("]").quoted(language)?),
            TokenKind::Plus => Ok(String::from("+").quoted(language)?),
            TokenKind::Minus => Ok(String::from(".").quoted(language)?),
            TokenKind::Asterisk => Ok(String::from("*").quoted(language)?),
            TokenKind::Slash => Ok(String::from("/").quoted(language)?),
            TokenKind::Caret => Ok(String::from("^").quoted(language)?),
            TokenKind::Separator => TranslatedMessage::new(
                "token_kind.seperator",
                translation_resolver,
            )
            .translate(language),
            TokenKind::Arrow => Ok(String::from("->").quoted(language)?),
            TokenKind::Define => Ok(String::from(":=").quoted(language)?),
            TokenKind::LessThan => Ok(String::from("<").quoted(language)?),
            TokenKind::LessThanOrEquals => {
                Ok(String::from("<=").quoted(language)?)
            }
            TokenKind::GreaterThan => Ok(String::from(">").quoted(language)?),
            TokenKind::GreaterThanOrEquals => {
                Ok(String::from(">=").quoted(language)?)
            }
            TokenKind::Equals => Ok(String::from("=").quoted(language)?),
            TokenKind::And => Ok(String::from("&").quoted(language)?),
            TokenKind::Or => Ok(String::from("|").quoted(language)?),
        }
    }
}

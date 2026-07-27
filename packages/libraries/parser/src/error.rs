use language::Language;
use lexer::{Token, TokenKind};
use node::NodeType;
use thiserror::Error;
use translate_core::{
    Quoted, Translatable, TranslatableList, TranslatedMessage, TranslationError,
};

use crate::translations::translation_resolver;

// TODO: add input spans for tracing
#[derive(Debug, Clone, Error, PartialEq)]
#[error("ParseError")]
pub enum Error {
    UnexpectedToken {
        expected: Vec<TokenKind>,
        actual: Token,
    },
    UnexpectedEndOfInput,
    UnexpectedLeftSideOfDefinition {
        node_type: NodeType,
    },
    InvalidFunctionArgumentDeclaration,
    InvalidFunctionName,
    Lexer(lexer::Error),
}

impl From<lexer::Error> for Error {
    fn from(value: lexer::Error) -> Self {
        Self::Lexer(value)
    }
}

impl Translatable for Error {
    fn translate(
        &self,
        language: Language,
    ) -> Result<String, TranslationError> {
        match self {
            Error::UnexpectedToken { expected, actual } => {
                let expected = expected.translate_list_or(language)?;
                let received = actual.to_string().quoted(language)?;
                TranslatedMessage::new(
                    "error.unexpected_token",
                    translation_resolver,
                )
                .key("expected", expected)
                .key("received", received)
                .translate(language)
            }
            Error::UnexpectedEndOfInput => TranslatedMessage::new(
                "error.unexpected_end_of_input",
                translation_resolver,
            )
            .translate(language),
            Error::UnexpectedLeftSideOfDefinition { .. } => {
                TranslatedMessage::new(
                    "error.unexpected_left_side_of_definition",
                    translation_resolver,
                )
                .translate(language)
            }
            Error::InvalidFunctionArgumentDeclaration => {
                TranslatedMessage::new(
                    "error.invalid_function_argument_declaration",
                    translation_resolver,
                )
                .translate(language)
            }
            Error::InvalidFunctionName => TranslatedMessage::new(
                "error.invalid_function_name",
                translation_resolver,
            )
            .translate(language),
            Error::Lexer(error) => error.translate(language),
        }
    }
}

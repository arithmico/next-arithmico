use common::Language;
use lexer::{GetTokenSpan, Span, Token, TokenKind};
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
    UnexpectedEndOfInput {
        span: Span,
    },
    UnexpectedLeftSideOfDefinition {
        node_type: NodeType,
        span: Span,
    },
    MissingMultiplyBetween {
        span: Span,
    },
    InvalidFunctionArgumentDeclaration {
        span: Span,
    },
    DuplicateFunctionArgumentName {
        name: String,
    },
    InvalidFunctionName,
    MissingClosingParenthesis {
        span: Span,
    },
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
                .key("position", actual.get_span().from.char_index + 1)
                .translate(language)
            }
            Error::UnexpectedEndOfInput { .. } => TranslatedMessage::new(
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
            Error::InvalidFunctionArgumentDeclaration { .. } => {
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
            Error::MissingMultiplyBetween { span } => TranslatedMessage::new(
                "error.missing_multiply_between",
                translation_resolver,
            )
            .key("from", span.from.char_index + 1)
            .key("to", span.to.char_index + 1)
            .translate(language),
            Error::MissingClosingParenthesis { span } => {
                TranslatedMessage::new(
                    "error.missing_closing_parenthesis",
                    translation_resolver,
                )
                .key("pos", span.from.char_index + 1)
                .translate(language)
            }
            Error::DuplicateFunctionArgumentName { name } => {
                TranslatedMessage::new(
                    "error.duplicate_function_argument_name",
                    translation_resolver,
                )
                .key("name", name)
                .translate(language)
            }
        }
    }
}

use lexer::{Token, TokenKind};
use node::NodeType;
use thiserror::Error;

// TODO: add input spans for tracing
#[derive(Debug, Clone, Error, PartialEq)]
#[error("ParseError")]
pub enum ParseError {
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

impl From<lexer::Error> for ParseError {
    fn from(value: lexer::Error) -> Self {
        Self::Lexer(value)
    }
}

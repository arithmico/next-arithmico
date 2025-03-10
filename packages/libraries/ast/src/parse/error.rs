use nom::error::ParseError;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
#[error("ParserError")]
pub struct ParseNodeError {}

impl<I> ParseError<I> for ParseNodeError {
    fn from_error_kind(_input: I, _kind: nom::error::ErrorKind) -> Self {
        Self {}
    }

    fn append(_input: I, _kind: nom::error::ErrorKind, _other: Self) -> Self {
        Self {}
    }
}

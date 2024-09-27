use nom::error::ParseError;

#[derive(Debug, Clone)]
pub struct ParserError {}

impl<I> ParseError<I> for ParserError {
    fn from_error_kind(_input: I, _kind: nom::error::ErrorKind) -> Self {
        Self {}
    }

    fn append(_input: I, _kind: nom::error::ErrorKind, _other: Self) -> Self {
        Self {}
    }
}

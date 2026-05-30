use crate::core::ParseNodeError;

pub trait MapErrorUtils<T> {
    fn map_parse_node_error(
        self,
        f: impl Fn(ParseNodeError) -> ParseNodeError,
    ) -> Result<T, nom::Err<ParseNodeError>>;
}

impl<T> MapErrorUtils<T> for Result<T, nom::Err<ParseNodeError>> {
    fn map_parse_node_error(
        self,
        f: impl Fn(ParseNodeError) -> ParseNodeError,
    ) -> Result<T, nom::Err<ParseNodeError>> {
        self.map_err(|err| match err {
            nom::Err::Incomplete(_) => err,
            nom::Err::Error(err) => nom::Err::Error(f(err)),
            nom::Err::Failure(err) => nom::Err::Failure(f(err)),
        })
    }
}

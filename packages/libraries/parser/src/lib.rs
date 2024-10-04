use ast::{Definition, Node};
use node::ParseNode;

mod cache;
mod error;
mod node;

pub use error::ParseNodeError;
use node::ParseResult;
use nom::combinator::all_consuming;

pub fn parse(input: &str) -> Result<Node, ParseNodeError> {
    all_consuming(Definition::parse)(input)
        .map(|(_, node)| node)
        .map_err(|error| match error {
            nom::Err::Incomplete(_) => {
                unreachable!("incomplete parser error")
            }
            nom::Err::Error(error) => error,
            nom::Err::Failure(error) => error,
        })
}

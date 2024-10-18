use ast::{Definition, Node};
use common::DecimalFormat;
use node::ParseNode;

mod cache;
mod error;
mod node;

pub use error::ParseNodeError;
use node::ParseResult;
use nom::combinator::all_consuming;

pub struct ParseNodeOptions {
    pub decimal_format: DecimalFormat,
}

impl ParseNodeOptions {
    pub fn new(decimal_format: DecimalFormat) -> Self {
        Self { decimal_format }
    }
}

pub fn parse(
    input: &str,
    options: ParseNodeOptions,
) -> Result<Node, ParseNodeError> {
    let input = match options.decimal_format {
        DecimalFormat::Comma => input.replace(",", ".").replace(";", ","),
        DecimalFormat::Dot => input.to_string(),
    };

    let result = all_consuming(Definition::parse)(&input)
        .map(|(_, node)| node)
        .map_err(|error| match error {
            nom::Err::Incomplete(_) => {
                unreachable!("incomplete parser error")
            }
            nom::Err::Error(error) => error,
            nom::Err::Failure(error) => error,
        });

    result
}

use cache::clear_cache;
use node::ParseNode;

mod cache;
mod error;
mod node;
mod trace;
mod with_parser;

pub use error::ParseNodeError;
use nom::{Parser, combinator::all_consuming};

use crate::{DecimalFormat, Definition, Node};

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

    let result = all_consuming(Definition::parse)
        .parse(&input)
        .map(|(_, node)| node)
        .map_err(|error| match error {
            nom::Err::Incomplete(_) => {
                unreachable!("incomplete parser error")
            }
            nom::Err::Error(error) => error,
            nom::Err::Failure(error) => error,
        });

    clear_cache();

    result
}

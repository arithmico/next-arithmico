use pest::{error::Error, Parser};

use crate::core::node::Node;

use super::{
    parser::{ArithmicoParser, Rule},
    transform::transform,
};

pub fn parse_statement(input: &str) -> Result<Node, Error<Rule>> {
    let pairs = ArithmicoParser::parse(Rule::statement, input)?
        .next()
        .unwrap();

    Ok(transform(pairs))
}

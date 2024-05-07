use pest::Parser;

use crate::node::Node;

use super::{
    parser::{ArithmicoParser, Rule},
    transform::transform,
};

pub fn parse_statement(input: &str) -> Node {
    let pairs = ArithmicoParser::parse(Rule::statement, input)
        .expect("failed to parse")
        .next()
        .unwrap();

    transform(pairs)
}

use ast::Node;
use nom::{
    bytes::complete::tag,
    character::complete::space0,
    sequence::{delimited, tuple},
};

use super::{ParseNode, ParseResult};

pub fn parse_sub_expression(input: &str) -> ParseResult {
    delimited(
        tuple((tag("("), space0)),
        Node::parse,
        tuple((space0, tag(")"))),
    )(input)
}

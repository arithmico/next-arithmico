use ast::Node;
use nom::{
    bytes::complete::tag,
    character::complete::space0,
    sequence::{delimited, tuple},
};

use crate::cache::with_cache;

use super::{ParseNode, ParseResult};

pub fn parse_sub_expression(input: &str) -> ParseResult {
    with_cache("parse_sub_expression", input, |input| {
        delimited(
            tuple((tag("("), space0)),
            Node::parse,
            tuple((space0, tag(")"))),
        )(input)
    })
}

use ast::Node;
use nom::{
    bytes::complete::tag,
    character::complete::space0,
    sequence::{delimited, tuple},
    IResult,
};

use super::ParseNode;

pub fn parse_sub_expression(input: &str) -> IResult<&str, Node> {
    delimited(
        tuple((tag("("), space0)),
        Node::parse,
        tuple((space0, tag(")"))),
    )(input)
}

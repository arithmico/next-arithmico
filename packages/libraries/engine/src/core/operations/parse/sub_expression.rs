use nom::{
    bytes::complete::tag, character::complete::space0, sequence::delimited,
    Parser,
};

use crate::core::Node;

use super::{with_parser, ParseNode, ParseResult};

pub fn parse_sub_expression(input: &'_ str) -> ParseResult<'_> {
    with_parser("parse_sub_expression", |input| {
        delimited((tag("("), space0), Node::parse, (space0, tag(")")))
            .parse(input)
    })(input)
}

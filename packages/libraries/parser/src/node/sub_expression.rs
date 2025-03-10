use ast::Node;
use nom::{
    bytes::complete::tag, character::complete::space0, sequence::delimited,
    Parser,
};

use crate::with_parser::with_parser;

use super::{ParseNode, ParseResult};

pub fn parse_sub_expression(input: &str) -> ParseResult {
    with_parser("parse_sub_expression", |input| {
        delimited((tag("("), space0), Node::parse, (space0, tag(")")))
            .parse(input)
    })(input)
}

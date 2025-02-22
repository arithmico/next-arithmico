use ast::{Boolean, Number, Symbol};
use nom::branch::alt;

use crate::with_parser::with_parser;

use super::{ParseNode, ParseResult, sub_expression::parse_sub_expression};

pub fn parse_literal(input: &str) -> ParseResult {
    with_parser("parse_literal", |input| {
        alt((
            Number::parse,
            Boolean::parse,
            Symbol::parse,
            parse_sub_expression,
        ))(input)
    })(input)
}

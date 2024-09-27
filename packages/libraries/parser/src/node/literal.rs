use ast::{Boolean, Number, Symbol};
use nom::branch::alt;

use crate::cache::with_cache;

use super::{sub_expression::parse_sub_expression, ParseNode, ParseResult};

pub fn parse_literal(input: &str) -> ParseResult {
    with_cache("parse_literal", input, |input| {
        alt((
            Number::parse,
            Boolean::parse,
            Symbol::parse,
            parse_sub_expression,
        ))(input)
    })
}

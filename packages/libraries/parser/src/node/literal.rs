use ast::{Boolean, Number, Symbol};
use nom::branch::alt;

use super::{sub_expression::parse_sub_expression, ParseNode, ParseResult};

pub fn parse_literal(input: &str) -> ParseResult {
    alt((
        Number::parse,
        Boolean::parse,
        Symbol::parse,
        parse_sub_expression,
    ))(input)
}

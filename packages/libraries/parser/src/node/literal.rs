use ast::{Boolean, Number, Symbol};
use nom::{branch::alt, Parser};

use crate::with_parser::with_parser;

use super::{sub_expression::parse_sub_expression, ParseNode, ParseResult};

pub fn parse_literal(input: &str) -> ParseResult {
    with_parser("parse_literal", |input| {
        alt((
            Number::parse,
            Boolean::parse,
            Symbol::parse,
            parse_sub_expression,
        ))
        .parse(input)
    })(input)
}

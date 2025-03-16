use nom::{branch::alt, Parser};

use crate::core::{Boolean, Number, Symbol};

use super::{
    sub_expression::parse_sub_expression, with_parser, ParseNode, ParseResult,
};

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

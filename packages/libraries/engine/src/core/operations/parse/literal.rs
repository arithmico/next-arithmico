use nom::{Parser, branch::alt};

use crate::core::{Boolean, Number, Symbol};

use super::{
    ParseNode, ParseResult, sub_expression::parse_sub_expression, with_parser,
};

pub fn parse_literal(input: &'_ str) -> ParseResult<'_> {
    with_parser("parse_literal", |input| {
        alt((
            parse_sub_expression,
            Number::parse,
            Boolean::parse,
            Symbol::parse,
        ))
        .parse(input)
    })(input)
}

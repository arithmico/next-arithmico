use nom::{branch::alt, Parser};

use node::{Boolean, Number, Symbol};

use super::{
    sub_expression::parse_sub_expression, with_parser, ParseNode, ParseResult,
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

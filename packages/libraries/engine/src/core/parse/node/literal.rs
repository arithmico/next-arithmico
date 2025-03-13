use nom::{Parser, branch::alt};

use crate::{Boolean, Number, Symbol, core::parse::with_parser::with_parser};

use super::{ParseNode, ParseResult, sub_expression::parse_sub_expression};

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

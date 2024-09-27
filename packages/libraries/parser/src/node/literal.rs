use ast::{Boolean, Node, Number, Symbol};
use nom::{branch::alt, error::VerboseError, IResult};

use super::{sub_expression::parse_sub_expression, ParseNode};

pub fn parse_literal(input: &str) -> IResult<&str, Node, VerboseError<&str>> {
    alt((
        Number::parse,
        Boolean::parse,
        Symbol::parse,
        parse_sub_expression,
    ))(input)
}

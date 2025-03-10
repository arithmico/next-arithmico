mod and;
mod boolean;
mod definition;
mod division;
mod function;
mod function_call;
mod literal;
mod negate;
mod number;
mod or;
mod power;
mod product;
mod relation;
mod sub_expression;
mod sum;
mod symbol;
mod tensor;

use nom::{branch::alt, IResult, Parser};
use sub_expression::parse_sub_expression;

use crate::{Function, Node};

use super::ParseNodeError;

pub type ParseResult<'a> = IResult<&'a str, Node, ParseNodeError>;

pub trait ParseNode {
    fn parse(input: &str) -> ParseResult;
}

impl ParseNode for Node {
    fn parse(input: &str) -> ParseResult {
        alt((Function::parse, parse_sub_expression)).parse(input)
    }
}

mod and;
mod boolean;
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

use ast::{Function, Node};
use nom::{branch::alt, IResult};
use sub_expression::parse_sub_expression;

use crate::error::ParserError;

pub type ParseResult<'a> = IResult<&'a str, Node, ParserError>;

pub trait ParseNode {
    fn parse(input: &str) -> ParseResult;
}

impl ParseNode for Node {
    fn parse(input: &str) -> ParseResult {
        alt((Function::parse, parse_sub_expression))(input)
    }
}

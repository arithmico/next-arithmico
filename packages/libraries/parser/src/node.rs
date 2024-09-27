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
use nom::{branch::alt, error::VerboseError, IResult};
use sub_expression::parse_sub_expression;

pub type ParseResult<'a> = IResult<&'a str, Node, VerboseError<&'a str>>;

pub trait ParseNode {
    fn parse(input: &str) -> ParseResult;
}

impl ParseNode for Node {
    fn parse(input: &str) -> IResult<&str, Node, VerboseError<&str>> {
        alt((Function::parse, parse_sub_expression))(input)
    }
}

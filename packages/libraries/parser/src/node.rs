mod and;
mod boolean;
mod division;
mod function;
mod function_call;
mod negate;
mod number;
mod or;
mod power;
mod product;
mod sum;
mod symbol;
mod tensor;

use ast::{Boolean, Function, Node, Number, Symbol};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space0,
    sequence::{delimited, tuple},
    IResult,
};

pub trait ParseNode {
    fn parse(input: &str) -> IResult<&str, Node>;
}

impl ParseNode for Node {
    fn parse(input: &str) -> IResult<&str, Node> {
        alt((Function::parse, parse_sub_expression))(input)
    }
}

fn parse_sub_expression(input: &str) -> IResult<&str, Node> {
    delimited(
        tuple((tag("("), space0)),
        Node::parse,
        tuple((space0, tag(")"))),
    )(input)
}

pub(crate) fn parse_literal(input: &str) -> IResult<&str, Node> {
    alt((
        Number::parse,
        Boolean::parse,
        Symbol::parse,
        parse_sub_expression,
    ))(input)
}

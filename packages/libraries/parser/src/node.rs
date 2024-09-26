mod boolean;
mod division;
mod function;
mod function_call;
mod negate;
mod number;
mod power;
mod product;
mod sum;
mod symbol;
mod tensor;

pub use boolean::*;
pub use division::*;
pub use function::*;
pub use function_call::*;
pub use negate::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space0,
    sequence::{delimited, tuple},
    IResult,
};
pub use number::*;
pub use power::*;
pub use product::*;
pub use sum::*;
pub use symbol::*;
pub use tensor::*;

#[derive(PartialEq, Debug, Clone)]
pub enum Node {
    Boolean(Boolean),
    Sum(Sum),
    Negate(Negate),
    Product(Product),
    Division(Division),
    Power(Power),
    Tensor(Tensor),
    Number(Number),
    Symbol(Symbol),
    Function(Function),
    FunctionCall(FunctionCall),
}

impl Node {
    pub fn parse(input: &str) -> IResult<&str, Node> {
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

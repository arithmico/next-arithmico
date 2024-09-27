use ast::{FunctionCall, Node, Symbol};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space0,
    combinator::{cut, opt},
    error::VerboseError,
    multi::many0,
    sequence::{delimited, preceded, tuple},
    IResult,
};

use crate::node::parse_sub_expression;

use super::{literal::parse_literal, ParseNode, ParseResult};

impl ParseNode for FunctionCall {
    fn parse(input: &str) -> ParseResult {
        alt((parse_function_call, parse_literal))(input)
    }
}

fn parse_function_call(input: &str) -> ParseResult {
    let (remaining_input, (target, arguments)) = tuple((
        parse_function_call_target,
        delimited(
            tuple((space0, tag("("), space0)),
            cut(opt(parse_function_call_arguments)),
            tuple((space0, tag(")"))),
        ),
    ))(input)?;

    Ok((
        remaining_input,
        FunctionCall::new(target, arguments.unwrap_or(Vec::new())),
    ))
}

fn parse_function_call_target(input: &str) -> ParseResult {
    alt((Symbol::parse, parse_sub_expression))(input)
}

fn parse_function_call_arguments(
    input: &str,
) -> IResult<&str, Vec<Node>, VerboseError<&str>> {
    let (remaining_input, (first, mut rest)) =
        tuple((Node::parse, many0(parse_function_call_arguments_item)))(input)?;

    rest.insert(0, first);
    Ok((remaining_input, rest))
}

fn parse_function_call_arguments_item(input: &str) -> ParseResult {
    preceded(tuple((space0, tag(","), space0)), Node::parse)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn function_empty_function_call() {
        let result = FunctionCall::parse("f()").unwrap();
        assert_eq!(result, ("", FunctionCall::new(Symbol::new("f"), vec![])));
    }

    #[test]
    fn function_function_call_1_argument() {
        let result = FunctionCall::parse("f(x)").unwrap();
        assert_eq!(
            result,
            (
                "",
                FunctionCall::new(Symbol::new("f"), vec![Symbol::new("x")])
            )
        );
    }

    #[test]
    fn function_function_call_2_arguments() {
        let result = FunctionCall::parse("f(x, y)").unwrap();
        assert_eq!(
            result,
            (
                "",
                FunctionCall::new(
                    Symbol::new("f"),
                    vec![Symbol::new("x"), Symbol::new("y")]
                )
            )
        );
    }

    #[test]
    fn parse_nested_function_call() {
        let result = FunctionCall::parse("f(f(f(x)))").unwrap();
        assert_eq!(
            result,
            (
                "",
                FunctionCall::new(
                    Symbol::new("f"),
                    vec![FunctionCall::new(
                        Symbol::new("f"),
                        vec![FunctionCall::new(
                            Symbol::new("f"),
                            vec![Symbol::new("x")]
                        )]
                    )]
                )
            )
        );
    }
}

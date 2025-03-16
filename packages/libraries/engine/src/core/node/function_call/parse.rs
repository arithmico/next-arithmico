use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space0,
    combinator::{cut, opt},
    multi::many0,
    sequence::{delimited, preceded},
    IResult, Parser,
};

use crate::core::{
    parse_literal, parse_sub_expression, with_parser, FunctionCall, Node,
    ParseNode, ParseNodeError, ParseResult, Symbol, TraceUtils,
};

impl ParseNode for FunctionCall {
    fn parse(input: &str) -> ParseResult {
        with_parser("FunctionCall::parse", |input| {
            alt((parse_function_call, parse_literal)).parse(input)
        })(input)
    }
}

fn parse_function_call(input: &str) -> ParseResult {
    let (remaining_input, (target, arguments)) = (
        parse_function_call_target,
        delimited(
            (space0, tag("("), space0),
            cut(opt(parse_function_call_arguments)),
            (space0, tag(")")),
        ),
    )
        .parse(input)?;

    Ok((
        remaining_input,
        FunctionCall::new(target, arguments.unwrap_or(Vec::new()))
            .with_span_from_parser(input, remaining_input),
    ))
}

fn parse_function_call_target(input: &str) -> ParseResult {
    alt((Symbol::parse, parse_sub_expression)).parse(input)
}

fn parse_function_call_arguments(
    input: &str,
) -> IResult<&str, Vec<Node>, ParseNodeError> {
    let (remaining_input, (first, mut rest)) =
        (Node::parse, many0(parse_function_call_arguments_item))
            .parse(input)?;

    rest.insert(0, first);
    Ok((remaining_input, rest))
}

fn parse_function_call_arguments_item(input: &str) -> ParseResult {
    preceded((space0, tag(","), space0), Node::parse).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use trace::TracableMut;

    #[test]
    fn function_empty_function_call() {
        let result = FunctionCall::parse("f()").unwrap();
        assert_eq!(
            result,
            (
                "",
                FunctionCall::new(Symbol::new("f").with_span(0, 0), vec![])
                    .with_span(0, 2)
            )
        );
    }

    #[test]
    fn function_function_call_1_argument() {
        let result = FunctionCall::parse("f(x)").unwrap();
        assert_eq!(
            result,
            (
                "",
                FunctionCall::new(
                    Symbol::new("f").with_span(0, 0),
                    vec![Symbol::new("x").with_span(2, 2)]
                )
                .with_span(0, 3)
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
                    Symbol::new("f").with_span(0, 0),
                    vec![
                        Symbol::new("x").with_span(2, 2),
                        Symbol::new("y").with_span(5, 5)
                    ]
                )
                .with_span(0, 6)
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
                    Symbol::new("f").with_span(0, 0),
                    vec![FunctionCall::new(
                        Symbol::new("f").with_span(2, 2),
                        vec![FunctionCall::new(
                            Symbol::new("f").with_span(4, 4),
                            vec![Symbol::new("x").with_span(6, 6)]
                        )
                        .with_span(4, 7)]
                    )
                    .with_span(2, 8)]
                )
                .with_span(0, 9)
            )
        );
    }
}

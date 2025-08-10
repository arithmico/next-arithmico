use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space0,
    combinator::{cut, opt},
    multi::many0,
    sequence::delimited,
    IResult, Parser,
};

use crate::core::{
    parse_raw_symbol, with_parser, FunctionSignature, Node, NodeType, Or,
    ParseNode, ParseNodeError, ParseResult, TraceUtils,
};

use super::Function;

impl ParseNode for Function {
    fn parse(input: &'_ str) -> ParseResult<'_> {
        with_parser("Function::parse", |input| {
            alt((parse_function, Or::parse)).parse(input)
        })(input)
    }
}

fn parse_function(input: &'_ str) -> ParseResult<'_> {
    let (remaining_input, (arguments, expression)) = (
        delimited(
            (tag("("), space0),
            opt(parse_function_arguments),
            (space0, tag(")"), space0, tag("->"), space0),
        ),
        cut(Node::parse),
    )
        .parse(input)?;

    let mut signature = FunctionSignature::new();
    for name in arguments.unwrap_or(vec![]) {
        signature
            .add_argument(name, |argument| argument.node_type(NodeType::Any));
    }
    signature = signature.add_return_type(NodeType::Any);

    Ok((
        remaining_input,
        Function::new(signature, expression)
            .with_span_from_parser(input, remaining_input),
    ))
}

fn parse_function_arguments(
    input: &str,
) -> IResult<&str, Vec<String>, ParseNodeError> {
    let (remaining_input, (first, mut rest)) =
        (parse_raw_symbol, many0(parse_function_argument_item)).parse(input)?;
    rest.insert(0, first);
    Ok((remaining_input, rest))
}

fn parse_function_argument_item(
    input: &str,
) -> IResult<&str, String, ParseNodeError> {
    let (remaining_input, (_, _, _, name)) =
        (space0, tag(","), space0, parse_raw_symbol).parse(input)?;

    Ok((remaining_input, name))
}

#[cfg(test)]
mod tests {
    use trace::TracableMut;

    use crate::core::{Number, Sum, Symbol};

    use super::*;

    #[test]
    fn parse_function_no_arguments() {
        let result = Function::parse("() -> 2").unwrap();
        let signature = FunctionSignature::new().add_return_type(NodeType::Any);

        assert_eq!(
            result,
            (
                "",
                Function::new(signature, Number::new(2.).with_span(6, 6))
                    .with_span(0, 6)
            )
        );
    }

    #[test]
    fn parse_function_1_argument() {
        let result = Function::parse("(x) -> x").unwrap();
        let signature = FunctionSignature::new()
            .argument("x", |argument| argument.node_type(NodeType::Any))
            .add_return_type(NodeType::Any);

        assert_eq!(
            result,
            (
                "",
                Function::new(signature, Symbol::new("x").with_span(7, 7))
                    .with_span(0, 7)
            )
        );
    }

    #[test]
    fn parse_function_2_arguments() {
        let result = Function::parse("(x, y) -> x + y").unwrap();
        let signature = FunctionSignature::new()
            .argument("x", |argument| argument.node_type(NodeType::Any))
            .argument("y", |argument| argument.node_type(NodeType::Any))
            .add_return_type(NodeType::Any);

        assert_eq!(
            result,
            (
                "",
                Function::new(
                    signature,
                    Sum::new(vec![
                        Symbol::new("x").with_span(10, 10),
                        Symbol::new("y").with_span(14, 14),
                    ])
                    .with_span(10, 14)
                )
                .with_span(0, 14)
            )
        );
    }

    #[test]
    fn parse_nested_functions() {
        let result = Function::parse("(x) -> (y) -> x + y").unwrap();
        assert_eq!(
            result,
            (
                "",
                Function::new(
                    FunctionSignature::new()
                        .argument("x", |argument| argument
                            .node_type(NodeType::Any))
                        .add_return_type(NodeType::Any),
                    Function::new(
                        FunctionSignature::new()
                            .argument("y", |argument| argument
                                .node_type(NodeType::Any))
                            .add_return_type(NodeType::Any),
                        Sum::new(vec![
                            Symbol::new("x").with_span(14, 14),
                            Symbol::new("y").with_span(18, 18),
                        ])
                        .with_span(14, 18)
                    )
                    .with_span(7, 18)
                )
                .with_span(0, 18)
            )
        );
    }
}

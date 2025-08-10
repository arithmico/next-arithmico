use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space0,
    combinator::cut,
    multi::separated_list0,
    sequence::{delimited, terminated},
    Parser,
};

use crate::core::{
    parse_raw_symbol, with_parser, Definition, Function, FunctionSignature,
    Node, NodeType, ParseNode, ParseResult, TraceUtils,
};

impl ParseNode for Definition {
    fn parse(input: &'_ str) -> ParseResult<'_> {
        with_parser("Definition::parse", |input| {
            alt((parse_define_function, parse_define_symbol, Node::parse))
                .parse(input)
        })(input)
    }
}

fn parse_define_symbol(input: &'_ str) -> ParseResult<'_> {
    let (remaining_input, (symbol, expression)) = (
        terminated(parse_raw_symbol, (space0, tag(":="), space0)),
        cut(Node::parse),
    )
        .parse(input)?;

    Ok((
        remaining_input,
        Definition::new(symbol, expression)
            .with_span_from_parser(input, remaining_input),
    ))
}

fn parse_define_function(input: &'_ str) -> ParseResult<'_> {
    let (remaining_input, ((symbol, arguments), expression)) = (
        terminated(
            (
                terminated(parse_raw_symbol, space0),
                delimited(
                    (tag("("), space0),
                    separated_list0(
                        (space0, tag(","), space0),
                        parse_raw_symbol,
                    ),
                    (space0, tag(")")),
                ),
            ),
            (space0, tag(":="), space0),
        ),
        cut(Node::parse),
    )
        .parse(input)?;

    let mut signature = FunctionSignature::new();
    for name in arguments {
        signature
            .add_argument(name, |argument| argument.node_type(NodeType::Any));
    }
    signature = signature.add_return_type(NodeType::Any);

    Ok((
        remaining_input,
        Definition::new(
            symbol,
            Function::new(signature, expression)
                .with_span_from_parser(input, remaining_input),
        )
        .with_span_from_parser(input, remaining_input),
    ))
}

#[cfg(test)]
mod tests {
    use trace::TracableMut;

    use crate::core::{Number, Power, Symbol};

    use super::*;

    #[test]
    fn parse_define_symbol() {
        let result = Definition::parse("a := 1").unwrap();
        assert_eq!(
            result,
            (
                "",
                Definition::new("a", Number::new(1.).with_span(5, 5))
                    .with_span(0, 5)
            )
        );
    }

    #[test]
    fn parse_define_function_no_arguments() {
        let result = Definition::parse("f() := 1").unwrap();
        assert_eq!(
            result,
            (
                "",
                Definition::new(
                    "f",
                    Function::new(
                        FunctionSignature::new().add_return_type(NodeType::Any),
                        Number::new(1.).with_span(7, 7)
                    )
                    .with_span(0, 7)
                )
                .with_span(0, 7)
            )
        );
    }

    #[test]
    fn parse_define_function_1_argument() {
        let result = Definition::parse("f(x) := x").unwrap();
        let signature = FunctionSignature::new()
            .argument("x", |argument| argument.node_type(NodeType::Any))
            .add_return_type(NodeType::Any);
        assert_eq!(
            result,
            (
                "",
                Definition::new(
                    "f",
                    Function::new(signature, Symbol::new("x").with_span(8, 8))
                        .with_span(0, 8)
                )
                .with_span(0, 8)
            )
        );
    }

    #[test]
    fn parse_define_function_2_argument() {
        let result = Definition::parse("f(x, y) := x^y").unwrap();
        let signature = FunctionSignature::new()
            .argument("x", |argument| argument.node_type(NodeType::Any))
            .argument("y", |argument| argument.node_type(NodeType::Any))
            .add_return_type(NodeType::Any);

        assert_eq!(
            result,
            (
                "",
                Definition::new(
                    "f",
                    Function::new(
                        signature,
                        Power::new(
                            Symbol::new("x").with_span(11, 11),
                            Symbol::new("y").with_span(13, 13),
                        )
                        .with_span(11, 13)
                    )
                    .with_span(0, 13)
                )
                .with_span(0, 13)
            )
        );
    }

    #[test]
    fn parse_define_function_2_argument_spaced() {
        let result = Definition::parse("f ( x, y ) := x ^ y").unwrap();
        let signature = FunctionSignature::new()
            .argument("x", |argument| argument.node_type(NodeType::Any))
            .argument("y", |argument| argument.node_type(NodeType::Any))
            .add_return_type(NodeType::Any);

        assert_eq!(
            result,
            (
                "",
                Definition::new(
                    "f",
                    Function::new(
                        signature,
                        Power::new(
                            Symbol::new("x").with_span(14, 14),
                            Symbol::new("y").with_span(18, 18),
                        )
                        .with_span(14, 18)
                    )
                    .with_span(0, 18)
                )
                .with_span(0, 18)
            )
        );
    }

    #[test]
    fn parse_define_function_2_no_space() {
        let result = Definition::parse("f(x,y):=x^y").unwrap();
        let signature = FunctionSignature::new()
            .argument("x", |argument| argument.node_type(NodeType::Any))
            .argument("y", |argument| argument.node_type(NodeType::Any))
            .add_return_type(NodeType::Any);

        assert_eq!(
            result,
            (
                "",
                Definition::new(
                    "f",
                    Function::new(
                        signature,
                        Power::new(
                            Symbol::new("x").with_span(8, 8),
                            Symbol::new("y").with_span(10, 10),
                        )
                        .with_span(8, 10)
                    )
                    .with_span(0, 10)
                )
                .with_span(0, 10)
            )
        );
    }
}

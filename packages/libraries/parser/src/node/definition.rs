use ast::{Definition, Function, Node};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space0,
    multi::separated_list0,
    sequence::{delimited, terminated, tuple},
};

use crate::{
    node::symbol::parse_raw_symbol, trace::TraceUtils, with_parser::with_parser,
};

use super::{ParseNode, ParseResult};

impl ParseNode for Definition {
    fn parse(input: &str) -> ParseResult {
        with_parser("Definition::parse", |input| {
            alt((parse_define_function, parse_define_symbol, Node::parse))(
                input,
            )
        })(input)
    }
}

fn parse_define_symbol(input: &str) -> ParseResult {
    let (remaining_input, (symbol, expression)) = tuple((
        terminated(parse_raw_symbol, tuple((space0, tag(":="), space0))),
        Node::parse,
    ))(input)?;

    Ok((
        remaining_input,
        Definition::new(symbol, expression)
            .with_span_from_parser(input, remaining_input),
    ))
}

fn parse_define_function(input: &str) -> ParseResult {
    let (remaining_input, ((symbol, arguments), expression)) = tuple((
        terminated(
            tuple((
                terminated(parse_raw_symbol, space0),
                delimited(
                    tuple((tag("("), space0)),
                    separated_list0(
                        tuple((space0, tag(","), space0)),
                        parse_raw_symbol,
                    ),
                    tuple((space0, tag(")"))),
                ),
            )),
            tuple((space0, tag(":="), space0)),
        ),
        Node::parse,
    ))(input)?;

    Ok((
        remaining_input,
        Definition::new(
            symbol,
            Function::new(arguments, expression)
                .with_span_from_parser(input, remaining_input),
        )
        .with_span_from_parser(input, remaining_input),
    ))
}

#[cfg(test)]
mod tests {
    use ast::{Number, Power, Symbol};
    use trace::TracableMut;

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
                    Function::new(vec![], Number::new(1.).with_span(7, 7))
                        .with_span(0, 7)
                )
                .with_span(0, 7)
            )
        );
    }

    #[test]
    fn parse_define_function_1_argument() {
        let result = Definition::parse("f(x) := x").unwrap();
        assert_eq!(
            result,
            (
                "",
                Definition::new(
                    "f",
                    Function::new(
                        vec!["x".into()],
                        Symbol::new("x").with_span(8, 8)
                    )
                    .with_span(0, 8)
                )
                .with_span(0, 8)
            )
        );
    }

    #[test]
    fn parse_define_function_2_argument() {
        let result = Definition::parse("f(x, y) := x^y").unwrap();
        assert_eq!(
            result,
            (
                "",
                Definition::new(
                    "f",
                    Function::new(
                        vec!["x".into(), "y".into()],
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
        assert_eq!(
            result,
            (
                "",
                Definition::new(
                    "f",
                    Function::new(
                        vec!["x".into(), "y".into()],
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
        assert_eq!(
            result,
            (
                "",
                Definition::new(
                    "f",
                    Function::new(
                        vec!["x".into(), "y".into()],
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

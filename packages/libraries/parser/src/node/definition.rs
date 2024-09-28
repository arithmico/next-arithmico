use ast::{Definition, Function, Node};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space0,
    multi::separated_list0,
    sequence::{delimited, terminated, tuple},
};

use crate::{cache::with_cache, node::symbol::parse_raw_symbol};

use super::{ParseNode, ParseResult};

impl ParseNode for Definition {
    fn parse(input: &str) -> ParseResult {
        with_cache("Definition::parse", input, |input| {
            alt((parse_define_function, parse_define_symbol, Node::parse))(
                input,
            )
        })
    }
}

fn parse_define_symbol(input: &str) -> ParseResult {
    let (remaining_input, (symbol, expression)) = tuple((
        terminated(parse_raw_symbol, tuple((space0, tag(":="), space0))),
        Node::parse,
    ))(input)?;

    Ok((remaining_input, Definition::new(symbol, expression)))
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
        Definition::new(symbol, Function::new(arguments, expression)),
    ))
}

#[cfg(test)]
mod tests {
    use ast::{Number, Power, Symbol};

    use super::*;

    #[test]
    fn parse_define_symbol() {
        let result = Definition::parse("a := 1").unwrap();
        assert_eq!(result, ("", Definition::new("a", Number::new(1.))));
    }

    #[test]
    fn parse_define_function_no_arguments() {
        let result = Definition::parse("f() := 1").unwrap();
        assert_eq!(
            result,
            (
                "",
                Definition::new("f", Function::new(vec![], Number::new(1.)))
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
                    Function::new(vec!["x".into()], Symbol::new("x"))
                )
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
                        Power::new(Symbol::new("x"), Symbol::new("y"),)
                    )
                )
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
                        Power::new(Symbol::new("x"), Symbol::new("y"),)
                    )
                )
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
                        Power::new(Symbol::new("x"), Symbol::new("y"),)
                    )
                )
            )
        );
    }
}

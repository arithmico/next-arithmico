use crate::with_parser::with_parser;
use ast::{Power, Tensor};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space0,
    multi::many1,
    sequence::{preceded, tuple},
};
use trace::{Tracable, TracableMut};

use super::{ParseNode, ParseResult};

impl ParseNode for Power {
    fn parse(input: &str) -> ParseResult {
        with_parser("Power::parse", |input| {
            alt((parse_power, parse_power_element))(input)
        })(input)
    }
}

fn parse_power(input: &str) -> ParseResult {
    let (remaining_input, (first, rest)) =
        tuple((parse_power_element, many1(parse_power_item)))(input)?;

    Ok((
        remaining_input,
        rest.into_iter().fold(first, |base, exponent| {
            let trace = base.trace().hull_trace(exponent.trace());
            Power::new(base, exponent).with_trace(trace)
        }),
    ))
}

fn parse_power_item(input: &str) -> ParseResult {
    let (remaining_input, node) = preceded(
        tuple((space0, tag("^"), space0)),
        parse_power_element,
    )(input)?;

    Ok((remaining_input, node))
}

fn parse_power_element(input: &str) -> ParseResult {
    Tensor::parse(input)
}

#[cfg(test)]
mod tests {
    use ast::{Number, Symbol};

    use super::*;

    #[test]
    fn parse_power() {
        let result = Power::parse("1 ^ 2").unwrap();
        assert_eq!(
            result,
            (
                "",
                Power::new(
                    Number::new(1.).with_span(0, 0),
                    Number::new(2.).with_span(4, 4)
                )
                .with_span(0, 4)
            )
        );
    }

    #[test]
    fn parse_chained_power() {
        let result = Power::parse("1 ^ 2 ^ 3").unwrap();
        assert_eq!(
            result,
            (
                "",
                Power::new(
                    Power::new(
                        Number::new(1.).with_span(0, 0),
                        Number::new(2.).with_span(4, 4)
                    )
                    .with_span(0, 4),
                    Number::new(3.).with_span(8, 8)
                )
                .with_span(0, 8)
            )
        );
    }

    #[test]
    fn parse_power_with_symbol() {
        let result = Power::parse("a ^ b").unwrap();
        assert_eq!(
            result,
            (
                "",
                Power::new(
                    Symbol::new("a").with_span(0, 0),
                    Symbol::new("b").with_span(4, 4)
                )
                .with_span(0, 4)
            )
        );
    }
}

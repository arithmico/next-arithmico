use nom::{
    branch::alt, character::complete::space0, combinator::cut, error::context,
    multi::many1, sequence::preceded, Parser,
};
use trace::{IntoTrace, TracableMut};

use crate::core::{expect_tag, with_parser, ParseNode, ParseResult, Tensor};

use super::Power;

impl ParseNode for Power {
    fn parse(input: &'_ str) -> ParseResult<'_> {
        with_parser("Power::parse", |input| {
            alt((context("power", parse_power), parse_power_element))
                .parse(input)
        })(input)
    }
}

fn parse_power(input: &'_ str) -> ParseResult<'_> {
    let (remaining_input, (first, rest)) =
        (parse_power_element, many1(parse_power_item)).parse(input)?;

    Ok((
        remaining_input,
        rest.into_iter().fold(first, |base, exponent| {
            let trace = (&base, &exponent).into_hull_trace();
            Power::new(base, exponent).with_trace(&trace)
        }),
    ))
}

fn parse_power_item(input: &'_ str) -> ParseResult<'_> {
    let (remaining_input, node) =
        preceded((space0, expect_tag("^"), space0), cut(parse_power_element))
            .parse(input)?;

    Ok((remaining_input, node))
}

fn parse_power_element(input: &'_ str) -> ParseResult<'_> {
    Tensor::parse(input)
}

#[cfg(test)]
mod tests {

    use crate::core::{Number, Symbol};

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

use ast::{Division, Power};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space0,
    multi::many1,
    sequence::{preceded, tuple},
};
use trace::{IntoTrace, TracableMut};

use crate::with_parser::with_parser;

use super::{ParseNode, ParseResult};

impl ParseNode for Division {
    fn parse(input: &str) -> ParseResult {
        with_parser("Division::parse", |input| {
            alt((parse_division, parse_division_element))(input)
        })(input)
    }
}

fn parse_division(input: &str) -> ParseResult {
    let (remaining_input, (first, rest)) =
        tuple((parse_division_element, many1(parse_division_item)))(input)?;

    Ok((
        remaining_input,
        rest.into_iter().fold(first, |dividend, divisor| {
            let trace = (&dividend, &divisor).into_hull_trace();
            Division::new(dividend, divisor).with_trace(&trace)
        }),
    ))
}

fn parse_division_item(input: &str) -> ParseResult {
    let (remaining_input, node) = preceded(
        tuple((space0, tag("/"), space0)),
        parse_division_element,
    )(input)?;

    Ok((remaining_input, node))
}

pub fn parse_division_element(input: &str) -> ParseResult {
    Power::parse(input)
}

#[cfg(test)]
mod tests {
    use ast::Number;

    use super::*;

    #[test]
    fn parse_division() {
        let result = Division::parse("1 / 2").unwrap();
        assert_eq!(
            result,
            (
                "",
                Division::new(
                    Number::new(1.).with_span(0, 0),
                    Number::new(2.).with_span(4, 4)
                )
                .with_span(0, 4)
            )
        );
    }

    #[test]
    fn parse_division_with_power() {
        let result = Division::parse("1 / 2^3").unwrap();
        assert_eq!(
            result,
            (
                "",
                Division::new(
                    Number::new(1.).with_span(0, 0),
                    Power::new(
                        Number::new(2.).with_span(4, 4),
                        Number::new(3.).with_span(6, 6),
                    )
                    .with_span(4, 6)
                )
                .with_span(0, 6)
            )
        );
    }

    #[test]
    fn parse_chained_division() {
        let result = Division::parse("1 / 2 / 3").unwrap();
        assert_eq!(
            result,
            (
                "",
                Division::new(
                    Division::new(
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
}

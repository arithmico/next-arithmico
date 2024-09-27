use ast::{Division, Power};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::space0,
    multi::many1, sequence::tuple,
};

use crate::cache::with_cache;

use super::{ParseNode, ParseResult};

impl ParseNode for Division {
    fn parse(input: &str) -> ParseResult {
        with_cache("Division::parse", input, |input| {
            alt((parse_division, parse_division_element))(input)
        })
    }
}

fn parse_division(input: &str) -> ParseResult {
    let (remaining_input, (first, rest)) =
        tuple((parse_division_element, many1(parse_division_item)))(input)?;

    Ok((
        remaining_input,
        rest.into_iter()
            .fold(first, |dividend, divisor| Division::new(dividend, divisor)),
    ))
}

fn parse_division_item(input: &str) -> ParseResult {
    let (remaining_input, (_, _, _, node)) =
        tuple((space0, tag("/"), space0, parse_division_element))(input)?;

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
            ("", Division::new(Number::new(1.), Number::new(2.)))
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
                    Number::new(1.),
                    Power::new(Number::new(2.), Number::new(3.),)
                )
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
                    Division::new(Number::new(1.), Number::new(2.)),
                    Number::new(3.)
                )
            )
        );
    }
}

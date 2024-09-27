use ast::{Power, Tensor};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::space0,
    multi::many1, sequence::tuple,
};

use crate::cache::with_cache;

use super::{ParseNode, ParseResult};

impl ParseNode for Power {
    fn parse(input: &str) -> ParseResult {
        with_cache("Power::parse", input, |input| {
            alt((parse_power, parse_power_element))(input)
        })
    }
}

fn parse_power(input: &str) -> ParseResult {
    let (remaining_input, (first, rest)) =
        tuple((parse_power_element, many1(parse_power_item)))(input)?;

    Ok((
        remaining_input,
        rest.into_iter()
            .fold(first, |base, exponent| Power::new(base, exponent)),
    ))
}

fn parse_power_item(input: &str) -> ParseResult {
    let (remaining_input, (_, _, _, node)) =
        tuple((space0, tag("^"), space0, parse_power_element))(input)?;

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
        assert_eq!(result, ("", Power::new(Number::new(1.), Number::new(2.))));
    }

    #[test]
    fn parse_chained_power() {
        let result = Power::parse("1 ^ 2 ^ 3").unwrap();
        assert_eq!(
            result,
            (
                "",
                Power::new(
                    Power::new(Number::new(1.), Number::new(2.)),
                    Number::new(3.)
                )
            )
        );
    }

    #[test]
    fn parse_power_with_symbol() {
        let result = Power::parse("a ^ b").unwrap();
        assert_eq!(
            result,
            ("", Power::new(Symbol::new("a"), Symbol::new("b")))
        );
    }
}

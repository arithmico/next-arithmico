use ast::{Negate, Product, Sum};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space0,
    combinator::cut,
    multi::many1,
    sequence::{preceded, tuple},
};

use super::{ParseNode, ParseResult};

impl ParseNode for Sum {
    fn parse(input: &str) -> ParseResult {
        alt((parse_sum, Negate::parse, Product::parse))(input)
    }
}

fn parse_sum(input: &str) -> ParseResult {
    let (remaining_input, (first, mut rest)) = tuple((
        Product::parse,
        many1(alt((parse_sum_item, Negate::parse))),
    ))(input)?;

    let mut elements = vec![first];
    elements.append(&mut rest);
    Ok((remaining_input, Sum::new(elements)))
}

fn parse_sum_item(input: &str) -> ParseResult {
    let (remaining_input, element) = preceded(
        tuple((space0, tag("+"), space0)),
        cut(Product::parse),
    )(input)?;
    Ok((remaining_input, element))
}

#[cfg(test)]
mod tests {
    use ast::{Node, Number};
    use nom::{combinator::all_consuming, error::VerboseError};

    use super::*;

    #[test]
    fn parse_error_sum_1() {
        let result: Result<(&str, Node), nom::Err<VerboseError<&str>>> =
            all_consuming(Sum::parse)("1+");
        assert!(result.is_err());
    }

    #[test]
    fn parse_sum_2() {
        let result = Sum::parse("1+2").unwrap();
        assert_eq!(
            result,
            ("", Sum::new(vec![Number::new(1.), Number::new(2.)]))
        );
    }

    #[test]
    fn parse_sum_3() {
        let result = Sum::parse("1+2+3").unwrap();
        assert_eq!(
            result,
            (
                "",
                Sum::new(vec![
                    Number::new(1.),
                    Number::new(2.),
                    Number::new(3.)
                ])
            )
        );
    }

    #[test]
    fn parse_sum_3_space() {
        let result = Sum::parse("1+ 2  +   3").unwrap();
        assert_eq!(
            result,
            (
                "",
                Sum::new(vec![
                    Number::new(1.),
                    Number::new(2.),
                    Number::new(3.)
                ])
            )
        );
    }

    #[test]
    fn parse_sum_with_product() {
        let result = Sum::parse("1 + 2 * 3").unwrap();
        assert_eq!(
            result,
            (
                "",
                Sum::new(vec![
                    Number::new(1.),
                    Product::new(vec![Number::new(2.), Number::new(3.),])
                ])
            )
        )
    }

    #[test]
    fn parse_sum_with_negate() {
        let result = Sum::parse("1 + 2 - 3").unwrap();
        assert_eq!(
            result,
            (
                "",
                Sum::new(vec![
                    Number::new(1.),
                    Number::new(2.),
                    Negate::new(Number::new(3.))
                ])
            )
        )
    }
}

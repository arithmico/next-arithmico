use ast::{Division, Node, Product};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space0,
    combinator::cut,
    multi::many1,
    sequence::{preceded, tuple},
    IResult,
};

use super::ParseNode;

impl ParseNode for Product {
    fn parse(input: &str) -> IResult<&str, Node> {
        alt((parse_product, Division::parse))(input)
    }
}

pub fn parse_product(input: &str) -> IResult<&str, Node> {
    let (remaining_input, (first, mut rest)) =
        tuple((Division::parse, many1(parse_product_item)))(input)?;

    let mut elements = vec![first];
    elements.append(&mut rest);
    Ok((remaining_input, Product::new(elements)))
}

fn parse_product_item(input: &str) -> IResult<&str, Node> {
    let (remaining_input, element) = preceded(
        tuple((space0, tag("*"), space0)),
        cut(Division::parse),
    )(input)?;
    Ok((remaining_input, element))
}

#[cfg(test)]
mod tests {
    use ast::{Number, Sum, Symbol};
    use nom::combinator::all_consuming;

    use super::*;

    #[test]
    fn parse_error_product_1() {
        let result: Result<(&str, Node), nom::Err<nom::error::Error<&str>>> =
            all_consuming(Product::parse)("1*");
        assert!(result.is_err());
    }

    #[test]
    fn parse_product_2() {
        let result = Product::parse("1*2").unwrap();
        assert_eq!(
            result,
            ("", Product::new(vec![Number::new(1.), Number::new(2.)]))
        );
    }

    #[test]
    fn parse_product_3() {
        let result = Product::parse("1*2*3").unwrap();
        assert_eq!(
            result,
            (
                "",
                Product::new(vec![
                    Number::new(1.),
                    Number::new(2.),
                    Number::new(3.)
                ])
            )
        );
    }

    #[test]
    fn parse_product_3_space() {
        let result = Product::parse("1* 2  *   3").unwrap();
        assert_eq!(
            result,
            (
                "",
                Product::new(vec![
                    Number::new(1.),
                    Number::new(2.),
                    Number::new(3.)
                ])
            )
        );
    }

    #[test]
    fn parse_product_with_division() {
        let result = Product::parse("1 / 2 *3").unwrap();
        assert_eq!(
            result,
            (
                "",
                Product::new(vec![
                    Division::new(Number::new(1.), Number::new(2.)),
                    Number::new(3.)
                ])
            )
        );
    }

    #[test]
    fn parse_product_with_sums() {
        let result = Product::parse("(a + b) * (c + d)").unwrap();
        assert_eq!(
            result,
            (
                "",
                Product::new(vec![
                    Sum::new(vec![Symbol::new("a"), Symbol::new("b"),]),
                    Sum::new(vec![Symbol::new("c"), Symbol::new("d"),]),
                ])
            )
        );
    }
}

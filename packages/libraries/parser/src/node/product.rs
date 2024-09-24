use nom::{
    branch::alt, bytes::complete::tag, character::complete::space0,
    multi::many1, sequence::tuple, IResult,
};

use crate::{parse_number, Node};

#[derive(PartialEq, Debug, Clone)]
pub struct Product {
    pub elements: Vec<Node>,
}

impl Product {
    pub fn new(elements: Vec<Node>) -> Node {
        Node::Product(Self { elements })
    }

    pub fn parse(input: &str) -> IResult<&str, Node> {
        alt((parse_product, parse_product_element))(input)
    }
}

pub fn parse_product(input: &str) -> IResult<&str, Node> {
    let (remaining_input, (first, mut rest)) =
        tuple((parse_product_element, many1(parse_product_item)))(input)?;

    let mut elements = vec![first];
    elements.append(&mut rest);
    Ok((remaining_input, Product::new(elements)))
}

fn parse_product_item(input: &str) -> IResult<&str, Node> {
    let (remaining_input, (_, _, _, element)) =
        tuple((space0, tag("*"), space0, parse_product_element))(input)?;
    Ok((remaining_input, element))
}

pub fn parse_product_element(input: &str) -> IResult<&str, Node> {
    parse_number(input)
}

#[cfg(test)]
mod tests {
    use nom::combinator::all_consuming;

    use crate::Number;

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
}

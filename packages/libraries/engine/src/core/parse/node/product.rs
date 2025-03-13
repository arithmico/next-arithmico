use nom::{
    branch::alt, bytes::complete::tag, character::complete::space0,
    combinator::cut, multi::many1, sequence::preceded, Parser,
};

use crate::{
    core::parse::{trace::TraceUtils, with_parser::with_parser},
    Division, Product,
};

use super::{ParseNode, ParseResult};

impl ParseNode for Product {
    fn parse(input: &str) -> ParseResult {
        with_parser("Product::parse", |input| {
            alt((parse_product, Division::parse)).parse(input)
        })(input)
    }
}

pub fn parse_product(input: &str) -> ParseResult {
    let (remaining_input, (first, mut rest)) =
        (Division::parse, many1(parse_product_item)).parse(input)?;

    let mut elements = vec![first];
    elements.append(&mut rest);
    Ok((
        remaining_input,
        Product::new(elements).with_span_from_parser(input, remaining_input),
    ))
}

fn parse_product_item(input: &str) -> ParseResult {
    let (remaining_input, element) =
        preceded((space0, tag("*"), space0), cut(Division::parse))
            .parse(input)?;
    Ok((remaining_input, element))
}

#[cfg(test)]
mod tests {
    use crate::{Node, Number, Sum, Symbol};
    use nom::combinator::all_consuming;
    use trace::TracableMut;

    use crate::ParseNodeError;

    use super::*;

    #[test]
    fn parse_error_product_1() {
        let result: Result<(&str, Node), nom::Err<ParseNodeError>> =
            all_consuming(Product::parse).parse("1*");
        assert!(result.is_err());
    }

    #[test]
    fn parse_product_2() {
        let result = Product::parse("1*2").unwrap();
        assert_eq!(
            result,
            (
                "",
                Product::new(vec![
                    Number::new(1.).with_span(0, 0),
                    Number::new(2.).with_span(2, 2)
                ])
                .with_span(0, 2)
            )
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
                    Number::new(1.).with_span(0, 0),
                    Number::new(2.).with_span(2, 2),
                    Number::new(3.).with_span(4, 4)
                ])
                .with_span(0, 4)
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
                    Number::new(1.).with_span(0, 0),
                    Number::new(2.).with_span(3, 3),
                    Number::new(3.).with_span(10, 10)
                ])
                .with_span(0, 10)
            )
        );
    }

    #[test]
    fn parse_product_with_division() {
        let result = Product::parse("1 / 2 * 3").unwrap();
        assert_eq!(
            result,
            (
                "",
                Product::new(vec![
                    Division::new(
                        Number::new(1.).with_span(0, 0),
                        Number::new(2.).with_span(4, 4)
                    )
                    .with_span(0, 4),
                    Number::new(3.).with_span(8, 8)
                ])
                .with_span(0, 8)
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
                    Sum::new(vec![
                        Symbol::new("a").with_span(1, 1),
                        Symbol::new("b").with_span(5, 5),
                    ])
                    .with_span(1, 5),
                    Sum::new(vec![
                        Symbol::new("c").with_span(11, 11),
                        Symbol::new("d").with_span(15, 15),
                    ])
                    .with_span(11, 15),
                ])
                .with_span(0, 16)
            )
        );
    }
}

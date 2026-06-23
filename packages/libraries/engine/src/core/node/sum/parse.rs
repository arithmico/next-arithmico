use node::{Negate, Product, Sum};
use nom::{
    branch::alt, character::complete::space0, combinator::cut, error::context,
    multi::many1, sequence::preceded, Parser,
};

use crate::core::{
    expect_tag, with_parser, ParseNode, ParseResult, TraceUtils,
};

impl ParseNode for Sum {
    fn parse(input: &'_ str) -> ParseResult<'_> {
        with_parser("Sum::parse", |input| {
            alt((context("sum", parse_sum), Negate::parse, Product::parse))
                .parse(input)
        })(input)
    }
}

fn parse_sum(input: &'_ str) -> ParseResult<'_> {
    let (remaining_input, (first, mut rest)) = (
        Product::parse,
        many1(alt((parse_sum_item, preceded(space0, Negate::parse)))),
    )
        .parse(input)?;

    let mut elements = vec![first];
    elements.append(&mut rest);
    Ok((
        remaining_input,
        Sum::new(elements).with_span_from_parser(input, remaining_input),
    ))
}

fn parse_sum_item(input: &'_ str) -> ParseResult<'_> {
    let (remaining_input, element) =
        preceded((space0, expect_tag("+"), space0), cut(Product::parse))
            .parse(input)?;
    Ok((remaining_input, element))
}

#[cfg(test)]
mod tests {
    use node::{Node, Number};
    use nom::combinator::all_consuming;
    use trace::TracableMut;

    use crate::core::ParseNodeError;

    use super::*;

    #[test]
    fn parse_error_sum_1() {
        let result: Result<(&str, Node), nom::Err<ParseNodeError>> =
            all_consuming(Sum::parse).parse("1+");
        assert!(result.is_err());
    }

    #[test]
    fn parse_sum_2() {
        let result = Sum::parse("1+2").unwrap();
        assert_eq!(
            result,
            (
                "",
                Sum::new(vec![
                    Number::new_node(1.).with_span(0, 0),
                    Number::new_node(2.).with_span(2, 2)
                ])
                .with_span(0, 2)
            )
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
                    Number::new_node(1.).with_span(0, 0),
                    Number::new_node(2.).with_span(2, 2),
                    Number::new_node(3.).with_span(4, 4)
                ])
                .with_span(0, 4)
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
                    Number::new_node(1.).with_span(0, 0),
                    Number::new_node(2.).with_span(3, 3),
                    Number::new_node(3.).with_span(10, 10)
                ])
                .with_span(0, 10)
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
                    Number::new_node(1.).with_span(0, 0),
                    Product::new(vec![
                        Number::new_node(2.).with_span(4, 4),
                        Number::new_node(3.).with_span(8, 8),
                    ])
                    .with_span(4, 8)
                ])
                .with_span(0, 8)
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
                    Number::new_node(1.).with_span(0, 0),
                    Number::new_node(2.).with_span(4, 4),
                    Negate::new(Number::new_node(3.).with_span(8, 8))
                        .with_span(6, 8)
                ])
                .with_span(0, 8)
            )
        )
    }
}

use nom::{
    Parser, branch::alt, bytes::complete::tag, character::complete::space0,
    combinator::cut, multi::many1, sequence::preceded,
};

use crate::{
    Negate, Product, Sum,
    core::parse::{trace::TraceUtils, with_parser::with_parser},
};

use super::{ParseNode, ParseResult};

impl ParseNode for Sum {
    fn parse(input: &str) -> ParseResult {
        with_parser("Sum::parse", |input| {
            alt((parse_sum, Negate::parse, Product::parse)).parse(input)
        })(input)
    }
}

fn parse_sum(input: &str) -> ParseResult {
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

fn parse_sum_item(input: &str) -> ParseResult {
    let (remaining_input, element) =
        preceded((space0, tag("+"), space0), cut(Product::parse))
            .parse(input)?;
    Ok((remaining_input, element))
}

#[cfg(test)]
mod tests {
    use nom::combinator::all_consuming;
    use trace::TracableMut;

    use crate::{Node, Number, ParseNodeError};

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
                    Number::new(1.).with_span(0, 0),
                    Number::new(2.).with_span(2, 2)
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
                    Number::new(1.).with_span(0, 0),
                    Number::new(2.).with_span(2, 2),
                    Number::new(3.).with_span(4, 4)
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
                    Number::new(1.).with_span(0, 0),
                    Number::new(2.).with_span(3, 3),
                    Number::new(3.).with_span(10, 10)
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
                    Number::new(1.).with_span(0, 0),
                    Product::new(vec![
                        Number::new(2.).with_span(4, 4),
                        Number::new(3.).with_span(8, 8),
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
                    Number::new(1.).with_span(0, 0),
                    Number::new(2.).with_span(4, 4),
                    Negate::new(Number::new(3.).with_span(8, 8))
                        .with_span(6, 8)
                ])
                .with_span(0, 8)
            )
        )
    }
}

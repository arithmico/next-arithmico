use node::{Negate, Product};
use nom::{
    character::complete::space0, error::context, sequence::preceded, Parser,
};

use crate::core::{
    expect_tag, with_parser, ParseNode, ParseResult, TraceUtils,
};

impl ParseNode for Negate {
    fn parse(input: &'_ str) -> ParseResult<'_> {
        with_parser("Negate::parse", |input| {
            context("negate", parse_negate).parse(input)
        })(input)
    }
}

fn parse_negate(input: &'_ str) -> ParseResult<'_> {
    let (remaining_input, value) =
        preceded((space0, expect_tag("-"), space0), Product::parse)
            .parse(input)?;
    Ok((
        remaining_input,
        Negate::new(value)
            .with_trimmed_span_from_parser(input, remaining_input),
    ))
}

#[cfg(test)]
mod tests {
    use trace::TracableMut;

    use node::Number;

    use super::*;

    #[test]
    fn parse_negate_number() {
        let result = Negate::parse("-1").unwrap();
        assert_eq!(
            result,
            (
                "",
                Negate::new(Number::new_node(1.).with_span(1, 1)).with_span(0, 1)
            )
        );
    }

    #[test]
    fn parse_negate_number_space() {
        let result = Negate::parse(" -  1").unwrap();
        assert_eq!(
            result,
            (
                "",
                Negate::new(Number::new_node(1.).with_span(4, 4)).with_span(1, 4)
            )
        );
    }

    #[test]
    fn parse_negate_product() {
        let result = Negate::parse("-1*2").unwrap();
        assert_eq!(
            result,
            (
                "",
                Negate::new(
                    Product::new(vec![
                        Number::new_node(1.).with_span(1, 1),
                        Number::new_node(2.).with_span(3, 3)
                    ])
                    .with_span(1, 3)
                )
                .with_span(0, 3)
            )
        );
    }
}

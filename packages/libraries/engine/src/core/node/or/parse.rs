use node::{And, Or};
use nom::{
    Parser, branch::alt, character::complete::space0, error::context,
    multi::many1, sequence::preceded,
};

use crate::core::{
    ParseNode, ParseResult, TraceUtils, expect_tag, with_parser,
};

impl ParseNode for Or {
    fn parse(input: &'_ str) -> ParseResult<'_> {
        with_parser("Or::parse", |input| {
            alt((context("or", parse_or), And::parse)).parse(input)
        })(input)
    }
}

fn parse_or(input: &'_ str) -> ParseResult<'_> {
    let (remaining_input, (first, mut rest)) =
        (And::parse, many1(parse_or_item)).parse(input)?;

    rest.insert(0, first);
    Ok((
        remaining_input,
        Or::new(rest).with_span_from_parser(input, remaining_input),
    ))
}

fn parse_or_item(input: &'_ str) -> ParseResult<'_> {
    preceded((space0, expect_tag("|"), space0), And::parse).parse(input)
}

#[cfg(test)]
mod tests {
    use trace::TracableMut;

    use node::{Boolean, Symbol};

    use super::*;

    #[test]
    fn parse_or_2() {
        let result = Or::parse("a | true").unwrap();
        assert_eq!(
            result,
            (
                "",
                Or::new(vec![
                    Symbol::new("a").with_span(0, 0),
                    Boolean::new(true).with_span(4, 7)
                ])
                .with_span(0, 7)
            )
        );
    }

    #[test]
    fn parse_or_3() {
        let result = Or::parse("a | true | c").unwrap();
        assert_eq!(
            result,
            (
                "",
                Or::new(vec![
                    Symbol::new("a").with_span(0, 0),
                    Boolean::new(true).with_span(4, 7),
                    Symbol::new("c").with_span(11, 11)
                ])
                .with_span(0, 11)
            )
        );
    }

    #[test]
    fn parse_or_with_and() {
        let result = Or::parse("a | true & c").unwrap();
        assert_eq!(
            result,
            (
                "",
                Or::new(vec![
                    Symbol::new("a").with_span(0, 0),
                    And::new(vec![
                        Boolean::new(true).with_span(4, 7),
                        Symbol::new("c").with_span(11, 11)
                    ])
                    .with_span(4, 11)
                ])
                .with_span(0, 11)
            )
        );
    }
}

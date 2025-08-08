use nom::{
    branch::alt, bytes::complete::tag, character::complete::space0,
    combinator::cut, multi::many1, sequence::preceded, Parser,
};

use crate::core::{
    parse_relation, with_parser, And, ParseNode, ParseResult, Sum, TraceUtils,
};

impl ParseNode for And {
    fn parse(input: &'_ str) -> ParseResult<'_> {
        with_parser("And::parse", |input| {
            alt((parse_and, parse_relation)).parse(input)
        })(input)
    }
}

fn parse_and(input: &'_ str) -> ParseResult<'_> {
    let (remaining_input, (first, mut rest)) =
        (Sum::parse, many1(parse_and_item)).parse(input)?;

    rest.insert(0, first);
    Ok((
        remaining_input,
        And::new(rest).with_span_from_parser(input, remaining_input),
    ))
}

fn parse_and_item(input: &'_ str) -> ParseResult<'_> {
    preceded((space0, tag("&"), space0), cut(parse_relation)).parse(input)
}

#[cfg(test)]
mod tests {
    use trace::TracableMut;

    use crate::core::{Boolean, Symbol};

    use super::*;

    #[test]
    fn parse_and_2() {
        let result = And::parse("a & true").unwrap();
        assert_eq!(
            result,
            (
                "",
                And::new(vec![
                    Symbol::new("a").with_span(0, 0),
                    Boolean::new(true).with_span(4, 7)
                ])
                .with_span(0, 7)
            )
        );
    }

    #[test]
    fn parse_and_3() {
        let result = And::parse("a & true & c").unwrap();
        assert_eq!(
            result,
            (
                "",
                And::new(vec![
                    Symbol::new("a").with_span(0, 0),
                    Boolean::new(true).with_span(4, 7),
                    Symbol::new("c").with_span(11, 11)
                ])
                .with_span(0, 11)
            )
        );
    }
}

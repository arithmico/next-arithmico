use nom::{
    Parser, branch::alt, bytes::complete::tag, character::complete::space0,
    combinator::cut, multi::many1, sequence::preceded,
};

use crate::{
    And, Sum,
    core::parse::{trace::TraceUtils, with_parser::with_parser},
};

use super::{ParseNode, ParseResult, relation::parse_relation};

impl ParseNode for And {
    fn parse(input: &str) -> ParseResult {
        with_parser("And::parse", |input| {
            alt((parse_and, parse_relation)).parse(input)
        })(input)
    }
}

fn parse_and(input: &str) -> ParseResult {
    let (remaining_input, (first, mut rest)) =
        (Sum::parse, many1(parse_and_item)).parse(input)?;

    rest.insert(0, first);
    Ok((
        remaining_input,
        And::new(rest).with_span_from_parser(input, remaining_input),
    ))
}

fn parse_and_item(input: &str) -> ParseResult {
    preceded((space0, tag("&"), space0), cut(parse_relation)).parse(input)
}

#[cfg(test)]
mod tests {
    use trace::TracableMut;

    use crate::{Boolean, Symbol};

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

use nom::{
    branch::alt, bytes::complete::tag, character::complete::space0,
    multi::many1, sequence::preceded, Parser,
};

use crate::{
    parse::{trace::TraceUtils, with_parser::with_parser},
    And, Or,
};

use super::{ParseNode, ParseResult};

impl ParseNode for Or {
    fn parse(input: &str) -> ParseResult {
        with_parser("Or::parse", |input| {
            alt((parse_or, And::parse)).parse(input)
        })(input)
    }
}

fn parse_or(input: &str) -> ParseResult {
    let (remaining_input, (first, mut rest)) =
        (And::parse, many1(parse_or_item)).parse(input)?;

    rest.insert(0, first);
    Ok((
        remaining_input,
        Or::new(rest).with_span_from_parser(input, remaining_input),
    ))
}

fn parse_or_item(input: &str) -> ParseResult {
    preceded((space0, tag("|"), space0), And::parse).parse(input)
}

#[cfg(test)]
mod tests {
    use trace::TracableMut;

    use crate::{Boolean, Symbol};

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

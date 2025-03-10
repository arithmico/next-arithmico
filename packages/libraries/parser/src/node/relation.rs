use crate::{
    error::ParseNodeError, trace::TraceUtils, with_parser::with_parser,
};
use ast::{
    And, Equals, GreaterThan, GreaterThanOrEquals, LessThan, LessThanOrEquals,
    Node, Sum,
};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::space0,
    combinator::cut, multi::many1, sequence::delimited, IResult, Parser,
};
use trace::IntoTrace;
use trace::TracableMut;

use super::{ParseNode, ParseResult};

pub fn parse_relation(input: &str) -> ParseResult {
    with_parser("parse_relation", |input| {
        alt((parse_relation_chain, parse_relation_element)).parse(input)
    })(input)
}

fn parse_relation_chain(input: &str) -> ParseResult {
    let (remaining_input, (first, rest)) =
        (parse_relation_element, many1(parse_relation_chain_item))
            .parse(input)?;

    let (mut relations, _) = rest.into_iter().fold(
        (Vec::<Node>::new(), first),
        |(mut relations, left), (operator, right)| {
            let trace = (&left, &right).into_hull_trace();
            let next = right.clone();
            let relation = match operator {
                "=" => Equals::new(left, right),
                "<" => LessThan::new(left, right),
                ">" => GreaterThan::new(left, right),
                "<=" => LessThanOrEquals::new(left, right),
                ">=" => GreaterThanOrEquals::new(left, right),
                _ => {
                    unreachable!()
                }
            }
            .with_trace(&trace);
            relations.push(relation);
            (relations, next)
        },
    );

    if relations.len() == 1 {
        return Ok((remaining_input, relations.remove(0)));
    }

    Ok((
        remaining_input,
        And::new(relations).with_span_from_parser(input, remaining_input),
    ))
}

fn parse_relation_chain_item(
    input: &str,
) -> IResult<&str, (&str, Node), ParseNodeError> {
    (
        delimited(space0, parse_relation_operator, space0),
        cut(parse_relation_element),
    )
        .parse(input)
}

fn parse_relation_operator(input: &str) -> IResult<&str, &str, ParseNodeError> {
    alt((tag("="), tag("<="), tag(">="), tag("<"), tag(">"))).parse(input)
}

fn parse_relation_element(input: &str) -> ParseResult {
    Sum::parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::Symbol;

    #[test]
    fn parse_equals() {
        let result = parse_relation("a = b").unwrap();
        assert_eq!(
            result,
            (
                "",
                Equals::new(
                    Symbol::new("a").with_span(0, 0),
                    Symbol::new("b").with_span(4, 4)
                )
                .with_span(0, 4)
            )
        );
    }

    #[test]
    fn parse_less_than() {
        let result = parse_relation("a < b").unwrap();
        assert_eq!(
            result,
            (
                "",
                LessThan::new(
                    Symbol::new("a").with_span(0, 0),
                    Symbol::new("b").with_span(4, 4)
                )
                .with_span(0, 4)
            )
        );
    }

    #[test]
    fn greater_less_than() {
        let result = parse_relation("a > b").unwrap();
        assert_eq!(
            result,
            (
                "",
                GreaterThan::new(
                    Symbol::new("a").with_span(0, 0),
                    Symbol::new("b").with_span(4, 4)
                )
                .with_span(0, 4)
            )
        );
    }

    #[test]
    fn parse_less_than_or_equals() {
        let result = parse_relation("a <= b").unwrap();
        assert_eq!(
            result,
            (
                "",
                LessThanOrEquals::new(
                    Symbol::new("a").with_span(0, 0),
                    Symbol::new("b").with_span(5, 5)
                )
                .with_span(0, 5)
            )
        );
    }

    #[test]
    fn greater_less_than_or_equals() {
        let result = parse_relation("a >= b").unwrap();
        assert_eq!(
            result,
            (
                "",
                GreaterThanOrEquals::new(
                    Symbol::new("a").with_span(0, 0),
                    Symbol::new("b").with_span(5, 5)
                )
                .with_span(0, 5)
            )
        );
    }

    #[test]
    fn relation_chain() {
        let result = parse_relation("a < b <= c = d >= e > f").unwrap();
        assert_eq!(
            result,
            (
                "",
                And::new(vec![
                    LessThan::new(
                        Symbol::new("a").with_span(0, 0),
                        Symbol::new("b").with_span(4, 4)
                    )
                    .with_span(0, 4),
                    LessThanOrEquals::new(
                        Symbol::new("b").with_span(4, 4),
                        Symbol::new("c").with_span(9, 9)
                    )
                    .with_span(4, 9),
                    Equals::new(
                        Symbol::new("c").with_span(9, 9),
                        Symbol::new("d").with_span(13, 13)
                    )
                    .with_span(9, 13),
                    GreaterThanOrEquals::new(
                        Symbol::new("d").with_span(13, 13),
                        Symbol::new("e").with_span(18, 18)
                    )
                    .with_span(13, 18),
                    GreaterThan::new(
                        Symbol::new("e").with_span(18, 18),
                        Symbol::new("f").with_span(22, 22)
                    )
                    .with_span(18, 22),
                ])
                .with_span(0, 22)
            )
        );
    }
}

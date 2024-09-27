use ast::{
    And, Equals, GreaterThan, GreaterThanOrEquals, LessThan, LessThanOrEquals,
    Node, Sum,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space0,
    multi::many1,
    sequence::{delimited, tuple},
    IResult,
};

use super::ParseNode;

pub fn parse_relation(input: &str) -> IResult<&str, Node> {
    alt((parse_relation_chain, parse_relation_element))(input)
}

fn parse_relation_chain(input: &str) -> IResult<&str, Node> {
    let (remaining_input, (first, rest)) = tuple((
        parse_relation_element,
        many1(parse_relation_chain_item),
    ))(input)?;

    let (mut relations, _) = rest.into_iter().fold(
        (Vec::<Node>::new(), first),
        |(mut relations, left), (operator, right)| {
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
            };
            relations.push(relation);
            (relations, next)
        },
    );

    if relations.len() == 1 {
        return Ok((remaining_input, relations.remove(0)));
    }

    Ok((remaining_input, And::new(relations)))
}

fn parse_relation_chain_item(input: &str) -> IResult<&str, (&str, Node)> {
    tuple((
        delimited(space0, parse_relation_operator, space0),
        parse_relation_element,
    ))(input)
}

fn parse_relation_operator(input: &str) -> IResult<&str, &str> {
    alt((tag("="), tag("<="), tag(">="), tag("<"), tag(">")))(input)
}

fn parse_relation_element(input: &str) -> IResult<&str, Node> {
    Sum::parse(input)
}

#[cfg(test)]
mod tests {
    use ast::Symbol;

    use super::*;

    #[test]
    fn parse_equals() {
        let result = parse_relation("a = b").unwrap();
        assert_eq!(
            result,
            ("", Equals::new(Symbol::new("a"), Symbol::new("b")))
        );
    }

    #[test]
    fn parse_less_than() {
        let result = parse_relation("a < b").unwrap();
        assert_eq!(
            result,
            ("", LessThan::new(Symbol::new("a"), Symbol::new("b")))
        );
    }

    #[test]
    fn greater_less_than() {
        let result = parse_relation("a > b").unwrap();
        assert_eq!(
            result,
            ("", GreaterThan::new(Symbol::new("a"), Symbol::new("b")))
        );
    }

    #[test]
    fn parse_less_than_or_equals() {
        let result = parse_relation("a <= b").unwrap();
        assert_eq!(
            result,
            (
                "",
                LessThanOrEquals::new(Symbol::new("a"), Symbol::new("b"))
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
                GreaterThanOrEquals::new(Symbol::new("a"), Symbol::new("b"))
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
                    LessThan::new(Symbol::new("a"), Symbol::new("b")),
                    LessThanOrEquals::new(Symbol::new("b"), Symbol::new("c")),
                    Equals::new(Symbol::new("c"), Symbol::new("d")),
                    GreaterThanOrEquals::new(
                        Symbol::new("d"),
                        Symbol::new("e")
                    ),
                    GreaterThan::new(Symbol::new("e"), Symbol::new("f")),
                ])
            )
        );
    }
}

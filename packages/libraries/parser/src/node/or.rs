use ast::{And, Node, Or};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space0,
    multi::many1,
    sequence::{preceded, tuple},
    IResult,
};

use super::ParseNode;

impl ParseNode for Or {
    fn parse(input: &str) -> IResult<&str, Node> {
        alt((parse_or, And::parse))(input)
    }
}

fn parse_or(input: &str) -> IResult<&str, Node> {
    let (remaining_input, (first, mut rest)) =
        tuple((And::parse, many1(parse_or_item)))(input)?;

    rest.insert(0, first);
    Ok((remaining_input, Or::new(rest)))
}

fn parse_or_item(input: &str) -> IResult<&str, Node> {
    preceded(tuple((space0, tag("|"), space0)), And::parse)(input)
}

#[cfg(test)]
mod tests {
    use ast::{Boolean, Symbol};

    use super::*;

    #[test]
    fn parse_or_2() {
        let result = Or::parse("a | true").unwrap();
        assert_eq!(
            result,
            ("", Or::new(vec![Symbol::new("a"), Boolean::new(true)]))
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
                    Symbol::new("a"),
                    Boolean::new(true),
                    Symbol::new("c")
                ])
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
                    Symbol::new("a"),
                    And::new(vec![Boolean::new(true), Symbol::new("c")])
                ])
            )
        );
    }
}

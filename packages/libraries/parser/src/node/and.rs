use ast::{And, Node, Sum};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space0,
    combinator::cut,
    error::VerboseError,
    multi::many1,
    sequence::{preceded, tuple},
    IResult,
};

use super::{relation::parse_relation, ParseNode};

impl ParseNode for And {
    fn parse(input: &str) -> IResult<&str, Node, VerboseError<&str>> {
        alt((parse_and, parse_relation))(input)
    }
}

fn parse_and(input: &str) -> IResult<&str, Node, VerboseError<&str>> {
    let (remaining_input, (first, mut rest)) =
        tuple((Sum::parse, many1(parse_and_item)))(input)?;

    rest.insert(0, first);
    Ok((remaining_input, And::new(rest)))
}

fn parse_and_item(input: &str) -> IResult<&str, Node, VerboseError<&str>> {
    preceded(tuple((space0, tag("&"), space0)), cut(parse_relation))(input)
}

#[cfg(test)]
mod tests {
    use ast::{Boolean, Symbol};

    use super::*;

    #[test]
    fn parse_and_2() {
        let result = And::parse("a & true").unwrap();
        assert_eq!(
            result,
            ("", And::new(vec![Symbol::new("a"), Boolean::new(true)]))
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
                    Symbol::new("a"),
                    Boolean::new(true),
                    Symbol::new("c")
                ])
            )
        );
    }
}

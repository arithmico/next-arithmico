use nom::{
    branch::alt, bytes::complete::tag, character::complete::space0,
    multi::many1, sequence::tuple, IResult,
};

use crate::{parse_number, Node};

#[derive(PartialEq, Debug, Clone)]
pub struct Division {
    pub dividend: Box<Node>,
    pub divisor: Box<Node>,
}

impl Division {
    pub fn new(dividend: Node, divisor: Node) -> Node {
        Node::Division(Self {
            dividend: Box::new(dividend),
            divisor: Box::new(divisor),
        })
    }

    pub fn parse(input: &str) -> IResult<&str, Node> {
        alt((parse_division, parse_division_element))(input)
    }
}

pub fn parse_division(input: &str) -> IResult<&str, Node> {
    let (remaining_input, (first, rest)) =
        tuple((parse_division_element, many1(parse_division_item)))(input)?;

    Ok((
        remaining_input,
        rest.into_iter()
            .fold(first, |dividend, divisor| Division::new(dividend, divisor)),
    ))
}

fn parse_division_item(input: &str) -> IResult<&str, Node> {
    let (remaining_input, (_, _, _, node)) =
        tuple((space0, tag("/"), space0, parse_division_element))(input)?;

    Ok((remaining_input, node))
}

pub fn parse_division_element(input: &str) -> IResult<&str, Node> {
    parse_number(input)
}

#[cfg(test)]
mod tests {
    use crate::Number;

    use super::*;

    #[test]
    fn parse_division() {
        let result = Division::parse("1 / 2").unwrap();
        assert_eq!(
            result,
            ("", Division::new(Number::new(1.), Number::new(2.)))
        );
    }

    #[test]
    fn parse_chained_division() {
        let result = Division::parse("1 / 2 / 3").unwrap();
        assert_eq!(
            result,
            (
                "",
                Division::new(
                    Division::new(Number::new(1.), Number::new(2.)),
                    Number::new(3.)
                )
            )
        );
    }
}

use nom::{bytes::complete::tag, multi::many1, sequence::tuple, IResult};

use crate::{parse_number, Node};

#[derive(PartialEq, Debug, Clone)]
pub struct Sum {
    pub elements: Vec<Node>,
}

impl Sum {
    pub fn new(elements: Vec<Node>) -> Node {
        Node::Sum(Self { elements })
    }
}

pub fn parse_sum(input: &str) -> IResult<&str, Node> {
    let (remaining_input, (first, mut rest)) =
        tuple((parse_number, many1(parse_sum_item)))(input)?;

    let mut elements = vec![first];
    elements.append(&mut rest);
    Ok((remaining_input, Sum::new(elements)))
}

pub fn parse_sum_item(input: &str) -> IResult<&str, Node> {
    let (remaining_input, (_, element)) =
        tuple((tag("+"), parse_number))(input)?;
    Ok((remaining_input, element))
}

#[cfg(test)]
mod tests {
    use nom::error::ErrorKind;

    use crate::Number;

    use super::*;

    #[test]
    fn parse_error_sum_1() {
        let result: Result<(&str, Node), nom::Err<nom::error::Error<&str>>> =
            parse_sum("1+");
        assert_eq!(
            result,
            Err(nom::Err::Error(nom::error::Error::new(
                "",
                ErrorKind::Float
            )))
        );
    }

    #[test]
    fn parse_sum_2() {
        let result = parse_sum("1+2").unwrap();
        assert_eq!(
            result,
            ("", Sum::new(vec![Number::new(1.), Number::new(2.)]))
        );
    }

    #[test]
    fn parse_sum_3() {
        let result = parse_sum("1+2+3").unwrap();
        assert_eq!(
            result,
            (
                "",
                Sum::new(vec![
                    Number::new(1.),
                    Number::new(2.),
                    Number::new(3.)
                ])
            )
        );
    }
}

use nom::{
    bytes::complete::tag, character::complete::space0, sequence::tuple, IResult,
};

use crate::{Node, Product};

#[derive(PartialEq, Debug, Clone)]
pub struct Negate {
    pub value: Box<Node>,
}

impl Negate {
    pub fn new(value: Node) -> Node {
        Node::Negate(Self {
            value: Box::new(value),
        })
    }

    pub fn parse(input: &str) -> IResult<&str, Node> {
        parse_negate(input)
    }
}

fn parse_negate(input: &str) -> IResult<&str, Node> {
    let (remaining_input, (_, _, _, value)) =
        tuple((space0, tag("-"), space0, Product::parse))(input)?;
    Ok((remaining_input, Negate::new(value)))
}

#[cfg(test)]
mod tests {
    use crate::Number;

    use super::*;

    #[test]
    fn parse_negate_number() {
        let result = Negate::parse("-1").unwrap();
        assert_eq!(result, ("", Negate::new(Number::new(1.))));
    }

    #[test]
    fn parse_negate_number_space() {
        let result = Negate::parse(" -  1").unwrap();
        assert_eq!(result, ("", Negate::new(Number::new(1.))));
    }

    #[test]
    fn parse_negate_product() {
        let result = Negate::parse("-1*2").unwrap();
        assert_eq!(
            result,
            (
                "",
                Negate::new(Product::new(vec![
                    Number::new(1.),
                    Number::new(2.)
                ]))
            )
        );
    }
}

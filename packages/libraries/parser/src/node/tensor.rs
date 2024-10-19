use ast::{FunctionCall, Node, Tensor};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space0,
    combinator::cut,
    multi::separated_list0,
    sequence::{delimited, tuple},
};

use crate::cache::with_cache;

use super::{ParseNode, ParseResult};

impl ParseNode for Tensor {
    fn parse(input: &str) -> ParseResult {
        with_cache("Tensor::parse", input, |input| {
            alt((parse_empty_tensor, parse_tensor, FunctionCall::parse))(input)
        })
    }
}

fn parse_empty_tensor(input: &str) -> ParseResult {
    let (remaining_input, _) = tuple((tag("["), space0, tag("]")))(input)?;
    Ok((remaining_input, Tensor::new(vec![])))
}

fn parse_tensor(input: &str) -> ParseResult {
    let (remaining_input, elements) = delimited(
        tuple((tag("["), space0)),
        cut(separated_list0(
            tuple((space0, tag(","), space0)),
            Node::parse,
        )),
        tuple((space0, tag("]"))),
    )(input)?;

    Ok((remaining_input, Tensor::new(elements)))
}

#[cfg(test)]
mod tests {

    use ast::{Number, Trace};

    use super::*;

    #[test]
    fn parse_empty_tensor() {
        assert_eq!(
            Tensor::parse("[]").unwrap(),
            (
                "",
                Node::Tensor(Tensor {
                    elements: vec![],
                    shape: vec![0],
                    trace: Trace::new()
                })
            )
        )
    }

    #[test]
    fn parse_tensor_rank1() {
        assert_eq!(
            Tensor::parse("[1]").unwrap(),
            (
                "",
                Node::Tensor(Tensor {
                    elements: vec![Number::new(1.)],
                    shape: vec![1],
                    trace: Trace::new()
                })
            )
        )
    }

    #[test]
    fn parse_tensor_rank2() {
        assert_eq!(
            Tensor::parse("[[1],[2],[3]]").unwrap(),
            (
                "",
                Node::Tensor(Tensor {
                    elements: vec![
                        Number::new(1.),
                        Number::new(2.),
                        Number::new(3.)
                    ],
                    shape: vec![3, 1],
                    trace: Trace::new()
                })
            )
        )
    }

    #[test]
    fn parse_tensor_rank3() {
        assert_eq!(
            Tensor::parse("[[[1,2,3],[4,5,6]]]").unwrap(),
            (
                "",
                Node::Tensor(Tensor {
                    elements: vec![
                        Number::new(1.),
                        Number::new(2.),
                        Number::new(3.),
                        Number::new(4.),
                        Number::new(5.),
                        Number::new(6.),
                    ],
                    shape: vec![1, 2, 3],
                    trace: Trace::new()
                })
            )
        )
    }
}

use nom::{
    Parser, branch::alt, bytes::complete::tag, character::complete::space0,
    combinator::cut, multi::separated_list0, sequence::delimited,
};

use crate::{
    FunctionCall, Node, Tensor,
    core::parse::{trace::TraceUtils, with_parser::with_parser},
};

use super::{ParseNode, ParseResult};

impl ParseNode for Tensor {
    fn parse(input: &str) -> ParseResult {
        with_parser("Tensor::parse", |input| {
            alt((parse_empty_tensor, parse_tensor, FunctionCall::parse))
                .parse(input)
        })(input)
    }
}

fn parse_empty_tensor(input: &str) -> ParseResult {
    let (remaining_input, _) = (tag("["), space0, tag("]")).parse(input)?;
    Ok((
        remaining_input,
        Tensor::new(vec![]).with_span_from_parser(input, remaining_input),
    ))
}

fn parse_tensor(input: &str) -> ParseResult {
    let (remaining_input, elements) = delimited(
        (tag("["), space0),
        cut(separated_list0((space0, tag(","), space0), Node::parse)),
        (space0, tag("]")),
    )
    .parse(input)?;

    Ok((
        remaining_input,
        Tensor::new(elements).with_span_from_parser(input, remaining_input),
    ))
}

#[cfg(test)]
mod tests {
    use crate::Number;

    use super::*;
    use trace::TracableMut;
    use trace::Trace;

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
                .with_span(0, 1)
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
                    elements: vec![Number::new(1.).with_span(1, 1)],
                    shape: vec![1],
                    trace: Trace::new()
                })
                .with_span(0, 2)
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
                        Number::new(1.).with_span(2, 2),
                        Number::new(2.).with_span(6, 6),
                        Number::new(3.).with_span(10, 10)
                    ],
                    shape: vec![3, 1],
                    trace: Trace::new()
                })
                .with_span(0, 12)
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
                        Number::new(1.).with_span(3, 3),
                        Number::new(2.).with_span(5, 5),
                        Number::new(3.).with_span(7, 7),
                        Number::new(4.).with_span(11, 11),
                        Number::new(5.).with_span(13, 13),
                        Number::new(6.).with_span(15, 15),
                    ],
                    shape: vec![1, 2, 3],
                    trace: Trace::new()
                })
                .with_span(0, 18)
            )
        )
    }
}

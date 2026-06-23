use node::{FunctionCall, Node, Tensor};
use nom::{
    Parser, branch::alt, character::complete::space0, combinator::cut,
    error::context, multi::separated_list0, sequence::delimited,
};

use crate::core::{
    ParseNode, ParseResult, TraceUtils, expect_tag,
    with_parser,
};

impl ParseNode for Tensor {
    fn parse(input: &'_ str) -> ParseResult<'_> {
        with_parser("Tensor::parse", |input| {
            alt((
                context("tensor", alt((parse_empty_tensor, parse_tensor))),
                FunctionCall::parse,
            ))
            .parse(input)
        })(input)
    }
}

fn parse_empty_tensor(input: &'_ str) -> ParseResult<'_> {
    let (remaining_input, _) =
        (expect_tag("["), space0, expect_tag("]")).parse(input)?;
    Ok((
        remaining_input,
        Tensor::new(vec![]).with_span_from_parser(input, remaining_input),
    ))
}

fn parse_tensor(input: &'_ str) -> ParseResult<'_> {
    let (remaining_input, elements) = delimited(
        (expect_tag("["), space0),
        cut(separated_list0(
            (space0, expect_tag(","), space0),
            Node::parse,
        )),
        cut((space0, expect_tag("]"))),
    )
    .parse(input)?;

    Ok((
        remaining_input,
        Tensor::new(elements).with_span_from_parser(input, remaining_input),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use node::Number;
    use std::assert_matches;
    use trace::TracableMut;
    use trace::Trace;

    use crate::core::{ParseNodeError, parse};

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
                    elements: vec![Number::new_node(1.).with_span(1, 1)],
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
                        Number::new_node(1.).with_span(2, 2),
                        Number::new_node(2.).with_span(6, 6),
                        Number::new_node(3.).with_span(10, 10)
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
                        Number::new_node(1.).with_span(3, 3),
                        Number::new_node(2.).with_span(5, 5),
                        Number::new_node(3.).with_span(7, 7),
                        Number::new_node(4.).with_span(11, 11),
                        Number::new_node(5.).with_span(13, 13),
                        Number::new_node(6.).with_span(15, 15),
                    ],
                    shape: vec![1, 2, 3],
                    trace: Trace::new()
                })
                .with_span(0, 18)
            )
        )
    }

    #[test]
    fn error_missing_closing_parenthesis() {
        let error = parse("[1, 2 + 4", &Default::default()).unwrap_err();

        assert_matches!(
            error,
            ParseNodeError::MissingParenthesis { round, square } if round == 0 && square == 1
        );
    }

    #[test]
    fn error_nested_missing_closing_parenthesis() {
        let error = parse("[1, [2 + 4]", &Default::default()).unwrap_err();

        assert_matches!(
            error,
            ParseNodeError::MissingParenthesis { round, square } if round == 0 && square == 1
        );
    }

    #[test]
    fn error_missing_opening_parenthesis() {
        let error = parse("1, 2 + 4] + 3", &Default::default()).unwrap_err();

        dbg!(&error);

        if let ParseNodeError::MissingParenthesis { square, round } = error
            && round == 0
            && square == -1
        {
            return;
        } else {
            panic!("invalid error");
        }
    }
}

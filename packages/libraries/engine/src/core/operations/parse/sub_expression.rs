use nom::{
    character::complete::space0, combinator::cut, error::context,
    sequence::delimited, Parser,
};

use crate::core::{operations::parse::tag::expect_tag, Node};

use super::{with_parser, ParseNode, ParseResult};

pub fn parse_sub_expression(input: &'_ str) -> ParseResult<'_> {
    with_parser("parse_sub_expression", |input| {
        let mut cx = context(
            "sub_expression",
            delimited(
                (expect_tag("("), space0),
                cut(Node::parse),
                cut(context("closing_parenthesis", (space0, expect_tag(")")))),
            ),
        );
        cx.parse(input)
    })(input)
}

#[cfg(test)]
mod tests {
    use core::panic;

    use crate::core::{parse, ParseNodeError};

    #[test]
    fn error_missing_closing_parenthesis() {
        let error = parse("1 + (a + b", &Default::default()).unwrap_err();

        if let ParseNodeError::Context { context, inner } = error
            && let Some(first) = context.first()
            && first.context == "closing_parenthesis"
            && first.input == ""
            && let ParseNodeError::LeafWithExpectation { expectation, input } =
                *inner
            && expectation == ")"
            && input == ""
        {
            return;
        } else {
            panic!("invalid error");
        }
    }
}

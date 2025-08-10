use nom::{
    bytes::complete::tag, character::complete::space0, combinator::cut,
    error::context, sequence::delimited, Parser,
};

use crate::core::Node;

use super::{with_parser, ParseNode, ParseResult};

pub fn parse_sub_expression(input: &'_ str) -> ParseResult<'_> {
    with_parser("parse_sub_expression", |input| {
        let mut cx = context(
            "sub_expression",
            delimited(
                (tag("("), space0),
                cut(Node::parse),
                cut(context("closing_parenthesis", (space0, tag(")")))),
            ),
        );
        cx.parse(input)
    })(input)
}

#[cfg(test)]
mod tests {
    use crate::core::parse;

    use super::*;

    #[test]
    fn error_missing_closing_parenthesis() {
        let error = parse("1 + (a + b", &Default::default()).unwrap_err();
        dbg!(error);
    }
}

use nom::{Parser, branch::alt, error::context};

use crate::core::{
    Boolean, Expectation, ParseNode, ParseNodeError, ParseResult, TraceUtils,
    expect_tag, map_parse_error::MapErrorUtils, with_parser,
};

impl ParseNode for Boolean {
    fn parse(input: &'_ str) -> ParseResult<'_> {
        with_parser("Boolean::parse", |input| parse_boolean(input))(input)
    }
}

fn parse_boolean(input: &'_ str) -> ParseResult<'_> {
    let (remaining_input, value) =
        context("boolean", alt((expect_tag("true"), expect_tag("false"))))
            .parse(input)
            .map_parse_node_error(|_| ParseNodeError::LeafWithExpectation {
                input: input.to_string(),
                expectation: Expectation::Boolean,
                actual: input.chars().next(),
            })?;

    Ok((
        remaining_input,
        Boolean::new(value == "true")
            .with_span_from_parser(input, remaining_input),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use trace::TracableMut;

    #[test]
    fn parse_true() {
        let result = Boolean::parse("true").unwrap();
        assert_eq!(result, ("", Boolean::new(true).with_span(0, 3)));
    }

    #[test]
    fn parse_false() {
        let result = Boolean::parse("false").unwrap();
        assert_eq!(result, ("", Boolean::new(false).with_span(0, 4)));
    }
}

use node::Number;
use nom::{Parser, error::context, number::complete::double};

use crate::core::{
    Expectation, ParseNode, ParseNodeError, ParseResult, TraceUtils,
    map_parse_error::MapErrorUtils, with_parser,
};

impl ParseNode for Number {
    fn parse(input: &'_ str) -> ParseResult<'_> {
        with_parser("Number::parse", |input| {
            context("number", parse_number).parse(input)
        })(input)
    }
}

fn parse_number(input: &'_ str) -> ParseResult<'_> {
    let (remaining_input, value) =
        double(input).map_parse_node_error(|_| {
            ParseNodeError::LeafWithExpectation {
                input: input.to_string(),
                expectation: Expectation::Number,
                actual: input.chars().next(),
            }
        })?;

    Ok((
        remaining_input,
        Number::new_node(value).with_span_from_parser(input, remaining_input),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use trace::TracableMut;

    #[test]
    fn parse_int() {
        let result = Number::parse("123").unwrap();
        assert_eq!(result, ("", Number::new_node(123.0).with_span(0, 2)));
    }

    #[test]
    fn parse_float() {
        let result = Number::parse("1.23").unwrap();
        assert_eq!(result, ("", Number::new_node(1.23).with_span(0, 3)));
    }
}

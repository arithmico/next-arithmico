use nom::{Compare, CompareResult, IResult, Input};

use crate::core::{Expectation, ParseNodeError};

pub fn expect_tag(
    tag: &'static str,
) -> impl Clone + Fn(&str) -> IResult<&str, &str, ParseNodeError> {
    let tag_len = tag.input_len();

    move |input: &str| match input.compare(tag) {
        CompareResult::Ok => Ok(input.take_split(tag_len)),
        _ => Err(nom::Err::Error(ParseNodeError::new_leaf_with_expectation(
            input,
            Expectation::Token(tag.to_string()),
            input.chars().next(),
        ))),
    }
}

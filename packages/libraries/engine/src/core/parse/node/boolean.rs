use nom::{branch::alt, bytes::complete::tag, Parser};

use crate::{
    core::parse::{trace::TraceUtils, with_parser::with_parser},
    Boolean,
};

use super::{ParseNode, ParseResult};

impl ParseNode for Boolean {
    fn parse(input: &str) -> ParseResult {
        with_parser("Boolean::parse", |input| parse_boolean(input))(input)
    }
}

fn parse_boolean(input: &str) -> ParseResult {
    let (remaining_input, value) =
        alt((tag("true"), tag("false"))).parse(input)?;

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

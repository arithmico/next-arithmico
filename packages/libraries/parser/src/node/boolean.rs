use ast::Boolean;
use nom::{branch::alt, bytes::complete::tag, Parser};

use crate::cache::with_cache;

use super::{ParseNode, ParseResult};

impl ParseNode for Boolean {
    fn parse(input: &str) -> ParseResult {
        with_cache("Boolean::parse", input, |input| parse_boolean(input))
    }
}

fn parse_boolean(input: &str) -> ParseResult {
    let (remaining_input, value) =
        alt((tag("true"), tag("false"))).parse(input)?;

    Ok((remaining_input, Boolean::new(value == "true")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_true() {
        let result = parse_boolean("true").unwrap();
        assert_eq!(result, ("", Boolean::new(true)));
    }

    #[test]
    fn parse_false() {
        let result = parse_boolean("false").unwrap();
        assert_eq!(result, ("", Boolean::new(false)));
    }
}

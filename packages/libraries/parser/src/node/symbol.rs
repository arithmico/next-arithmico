use ast::Symbol;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1},
    multi::many0,
    sequence::tuple,
    IResult,
};

use crate::{cache::with_cache, error::ParseNodeError};

use super::{ParseNode, ParseResult};

impl ParseNode for Symbol {
    fn parse(input: &str) -> ParseResult {
        with_cache("Symbol::parse", input, |input| parse_symbol(input))
    }
}

fn parse_symbol(input: &str) -> ParseResult {
    let (remaining_input, name) = parse_raw_symbol(input)?;
    Ok((remaining_input, Symbol::new(&name)))
}

pub fn parse_raw_symbol(input: &str) -> IResult<&str, String, ParseNodeError> {
    let (remaining_input, (start, rest)) =
        tuple((alpha1, many0(alt((alphanumeric1, tag("_"))))))(input)?;

    Ok((
        remaining_input,
        format!("{}{}", start, rest.into_iter().collect::<String>()),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_symbol_1() {
        let result = Symbol::parse("a").unwrap();
        assert_eq!(result, ("", Symbol::new("a")));
    }

    #[test]
    fn parse_symbol_only_alpha() {
        let result = Symbol::parse("abc").unwrap();
        assert_eq!(result, ("", Symbol::new("abc")));
    }

    #[test]
    fn parse_symbol_only_alphanumeric() {
        let result = Symbol::parse("abc123abc").unwrap();
        assert_eq!(result, ("", Symbol::new("abc123abc")));
    }

    #[test]
    fn parse_symbol_only_alphanumeric_with_underscores() {
        let result = Symbol::parse("ab_c1_23_abc").unwrap();
        assert_eq!(result, ("", Symbol::new("ab_c1_23_abc")));
    }
}

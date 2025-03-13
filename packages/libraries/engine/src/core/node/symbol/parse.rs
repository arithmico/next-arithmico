use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1},
    multi::many0,
    IResult, Parser,
};

use crate::{
    with_parser, ParseNode, ParseNodeError, ParseResult, Symbol, TraceUtils,
};

impl ParseNode for Symbol {
    fn parse(input: &str) -> ParseResult {
        with_parser("Symbol::parse", |input| parse_symbol(input))(input)
    }
}

fn parse_symbol(input: &str) -> ParseResult {
    let (remaining_input, name) = parse_raw_symbol(input)?;
    Ok((
        remaining_input,
        Symbol::new(&name).with_span_from_parser(input, remaining_input),
    ))
}

pub fn parse_raw_symbol(input: &str) -> IResult<&str, String, ParseNodeError> {
    let (remaining_input, (start, rest)) =
        (alpha1, many0(alt((alphanumeric1, tag("_"))))).parse(input)?;

    Ok((
        remaining_input,
        format!("{}{}", start, rest.into_iter().collect::<String>()),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use trace::TracableMut;

    #[test]
    fn parse_symbol_1() {
        let result = Symbol::parse("a").unwrap();
        assert_eq!(result, ("", Symbol::new("a").with_span(0, 0)));
    }

    #[test]
    fn parse_symbol_only_alpha() {
        let result = Symbol::parse("abc").unwrap();
        assert_eq!(result, ("", Symbol::new("abc").with_span(0, 2)));
    }

    #[test]
    fn parse_symbol_only_alphanumeric() {
        let result = Symbol::parse("abc123abc").unwrap();
        assert_eq!(result, ("", Symbol::new("abc123abc").with_span(0, 8)));
    }

    #[test]
    fn parse_symbol_only_alphanumeric_with_underscores() {
        let result = Symbol::parse("ab_c1_23_abc").unwrap();
        assert_eq!(result, ("", Symbol::new("ab_c1_23_abc").with_span(0, 11)));
    }
}

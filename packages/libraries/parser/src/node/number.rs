use ast::Number;
use nom::number::complete::double;

use crate::{trace::TraceUtils, with_parser::with_parser};

use super::{ParseNode, ParseResult};

impl ParseNode for Number {
    fn parse(input: &str) -> ParseResult {
        with_parser("Number::parse", |input| parse_number(input))(input)
    }
}

fn parse_number(input: &str) -> ParseResult {
    let (remaining_input, value) = double(input)?;

    Ok((
        remaining_input,
        Number::new(value).with_span_from_parser(input, remaining_input),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use trace::TracableMut;

    #[test]
    fn parse_int() {
        let result = Number::parse("123").unwrap();
        assert_eq!(result, ("", Number::new(123.0).with_span(0, 2)));
    }

    #[test]
    fn parse_float() {
        let result = Number::parse("1.23").unwrap();
        assert_eq!(result, ("", Number::new(1.23).with_span(0, 3)));
    }
}

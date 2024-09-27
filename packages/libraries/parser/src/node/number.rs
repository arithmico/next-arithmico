use ast::Number;
use nom::number::complete::double;

use super::{ParseNode, ParseResult};

impl ParseNode for Number {
    fn parse(input: &str) -> ParseResult {
        parse_number(input)
    }
}

fn parse_number(input: &str) -> ParseResult {
    let (remaining_input, value) = double(input)?;

    Ok((remaining_input, Number::new(value)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_int() {
        let result = parse_number("123").unwrap();
        assert_eq!(result, ("", Number::new(123.0)));
    }

    #[test]
    fn parse_float() {
        let result = parse_number("1.23").unwrap();
        assert_eq!(result, ("", Number::new(1.23)));
    }
}

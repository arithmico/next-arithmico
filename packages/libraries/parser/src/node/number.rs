use ast::{Node, Number};
use nom::{error::VerboseError, number::complete::double, IResult};

use super::ParseNode;

impl ParseNode for Number {
    fn parse(input: &str) -> IResult<&str, Node, VerboseError<&str>> {
        parse_number(input)
    }
}

fn parse_number(input: &str) -> IResult<&str, Node, VerboseError<&str>> {
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

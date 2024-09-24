use nom::{number::complete::double, IResult};

#[derive(PartialEq, Debug)]
pub struct Number {
    pub value: f64,
}

impl Number {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

pub fn parse_number(input: &str) -> IResult<&str, Number> {
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

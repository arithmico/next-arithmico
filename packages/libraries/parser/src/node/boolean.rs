use nom::{branch::alt, bytes::complete::tag, IResult, Parser};

#[derive(PartialEq, Debug)]
pub struct Boolean {
    pub value: bool,
}

impl Boolean {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
}

pub fn parse_boolean(input: &str) -> IResult<&str, Boolean> {
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

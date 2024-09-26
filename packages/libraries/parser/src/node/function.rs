use ast::{Function, Node, Sum};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::space0,
    combinator::opt, multi::many0, sequence::tuple, IResult,
};

use super::{parse_raw_symbol, ParseNode};

impl ParseNode for Function {
    fn parse(input: &str) -> IResult<&str, Node> {
        alt((parse_function, Sum::parse))(input)
    }
}

fn parse_function(input: &str) -> IResult<&str, Node> {
    let (remaining_input, (_, _, arguments, _, _, _, _, _, expression)) =
        tuple((
            tag("("),
            space0,
            opt(parse_function_arguments),
            space0,
            tag(")"),
            space0,
            tag("->"),
            space0,
            Node::parse,
        ))(input)?;

    Ok((
        remaining_input,
        Function::new(arguments.unwrap_or(vec![]), expression),
    ))
}

fn parse_function_arguments(input: &str) -> IResult<&str, Vec<String>> {
    let (remaining_input, (first, mut rest)) =
        tuple((parse_raw_symbol, many0(parse_function_argument_item)))(input)?;
    rest.insert(0, first);
    Ok((remaining_input, rest))
}

fn parse_function_argument_item(input: &str) -> IResult<&str, String> {
    let (remaining_input, (_, _, _, name)) =
        tuple((space0, tag(","), space0, parse_raw_symbol))(input)?;

    Ok((remaining_input, name))
}

#[cfg(test)]
mod tests {

    use ast::{Number, Symbol};

    use super::*;

    #[test]
    fn parse_function_no_arguments() {
        let result = Function::parse("() -> 2").unwrap();
        assert_eq!(result, ("", Function::new(vec![], Number::new(2.))));
    }

    #[test]
    fn parse_function_1_argument() {
        let result = Function::parse("(x) -> x").unwrap();
        assert_eq!(
            result,
            ("", Function::new(vec![String::from("x")], Symbol::new("x")))
        );
    }

    #[test]
    fn parse_function_2_arguments() {
        let result = Function::parse("(x, y) -> x + y").unwrap();
        assert_eq!(
            result,
            (
                "",
                Function::new(
                    vec![String::from("x"), String::from("y")],
                    Sum::new(vec![Symbol::new("x"), Symbol::new("y"),])
                )
            )
        );
    }

    #[test]
    fn parse_nested_functions() {
        let result = Function::parse("(x) -> (y) -> x + y").unwrap();
        assert_eq!(
            result,
            (
                "",
                Function::new(
                    vec![String::from("x")],
                    Function::new(
                        vec![String::from("y")],
                        Sum::new(vec![Symbol::new("x"), Symbol::new("y"),])
                    )
                )
            )
        );
    }
}

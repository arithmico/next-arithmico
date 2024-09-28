use ast::{Definition, Node};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space0,
    sequence::{terminated, tuple},
};

use crate::{cache::with_cache, node::symbol::parse_raw_symbol, ParseNode};

use super::ParseResult;

impl ParseNode for Definition {
    fn parse(input: &str) -> ParseResult {
        with_cache("Definition::parse", input, |input| {
            alt((parse_define_symbol, Node::parse))(input)
        })
    }
}

fn parse_define_symbol(input: &str) -> ParseResult {
    let (remaining_input, (symbol, expression)) = tuple((
        terminated(parse_raw_symbol, tuple((space0, tag(":="), space0))),
        Node::parse,
    ))(input)?;

    Ok((remaining_input, Definition::new(symbol, expression)))
}

#[cfg(test)]
mod tests {
    use ast::Number;

    use super::*;

    #[test]
    fn parse_define_symbol() {
        let result = Definition::parse("a := 1").unwrap();
        assert_eq!(result, ("", Definition::new("a", Number::new(1.))));
    }
}

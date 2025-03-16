use nom::{branch::alt, Parser};

use crate::core::{
    parse_sub_expression, Function, Node, ParseNode, ParseResult,
};

impl ParseNode for Node {
    fn parse(input: &str) -> ParseResult {
        alt((Function::parse, parse_sub_expression)).parse(input)
    }
}

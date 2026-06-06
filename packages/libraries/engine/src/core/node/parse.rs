use nom::{Parser, branch::alt};

use crate::core::{
    Function, Node, ParseNode, ParseResult, parse_sub_expression,
};

impl ParseNode for Node {
    fn parse(input: &'_ str) -> ParseResult<'_> {
        alt((Function::parse, parse_sub_expression)).parse(input)
    }
}

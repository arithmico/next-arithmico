use node::{Function, Node};
use nom::{branch::alt, Parser};

use crate::core::{parse_sub_expression, ParseNode, ParseResult};

impl ParseNode for Node {
    fn parse(input: &'_ str) -> ParseResult<'_> {
        alt((Function::parse, parse_sub_expression)).parse(input)
    }
}

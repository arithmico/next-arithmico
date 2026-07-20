use language::Language;
use lexer::tokenize;

mod binding_power;
mod cursor;
mod error;
mod expression;

pub use error::*;
use node::Node;

use crate::{cursor::Cursor, expression::parse_expression};

type ParseResult<'a, T> = Result<(Cursor<'a>, T), ParseError>;

pub fn parse(input: &str, language: Language) -> Result<Node, ParseError> {
    let tokens = tokenize(input, language)?;
    let cursor = Cursor::new(&tokens);
    let (mut cursor, node) = parse_expression(cursor)?;
    if let Some(token) = cursor.next() {
        Err(ParseError::UnexpectedToken {
            expected: vec![],
            actual: token.clone(),
        })
    } else {
        Ok(node)
    }
}

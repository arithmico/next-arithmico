use ast::Definition;
use node::ParseNode;

mod cache;
mod error;
mod node;

pub use error::ParserError;
pub use node::ParseResult;

pub fn parse(input: &str) -> ParseResult {
    Definition::parse(input)
}

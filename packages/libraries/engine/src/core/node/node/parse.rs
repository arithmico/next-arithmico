use crate::core::node::{Node, NodeError};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ArithmicoParser;
use pest::Parser;

impl Node {
    pub fn parse(input: &str) -> Result<Self, NodeError> {
        let pairs = ArithmicoParser::parse(Rule::statement, input)?
            .next()
            .unwrap();

        Ok(Self::try_from(pairs).unwrap())
    }
}

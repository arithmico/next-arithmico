use pest::iterators::Pair;

use crate::core::{
    node::{Node, NodeError},
    parse::Rule,
};

use super::Product;

impl TryFrom<Pair<'_, Rule>> for Product {
    type Error = NodeError;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::product => Ok(Product::new(
                pair.into_inner()
                    .map(|item| Node::try_from(item).unwrap())
                    .collect(),
            )),
            _ => Err(NodeError::ParsingError(String::from(
                "failed to convert parse tree to syntax tree",
            ))),
        }
    }
}

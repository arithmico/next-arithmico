use pest::iterators::Pair;

use crate::core::node::{
    errors::TransformParseTreeError, parse::parser::Rule, Node,
};

use super::Product;

impl TryFrom<Pair<'_, Rule>> for Product {
    type Error = TransformParseTreeError;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::product => Ok(Product::new(
                pair.into_inner()
                    .map(|item| Node::try_from(item).unwrap())
                    .collect(),
            )),
            _ => Err(TransformParseTreeError::ConversionFailed),
        }
    }
}

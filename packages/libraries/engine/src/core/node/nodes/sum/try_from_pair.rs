use pest::iterators::Pair;

use crate::core::{
    node::{errors::TransformParseTreeError, Node},
    parse::Rule,
};

use super::Sum;

impl TryFrom<Pair<'_, Rule>> for Sum {
    type Error = TransformParseTreeError;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::sum => Ok(Sum::new(
                pair.into_inner()
                    .map(|item| Node::try_from(item).unwrap())
                    .collect(),
            )),
            _ => Err(TransformParseTreeError::ConversionFailed),
        }
    }
}

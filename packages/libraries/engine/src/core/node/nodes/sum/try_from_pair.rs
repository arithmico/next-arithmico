use pest::iterators::Pair;

use crate::core::node::{Node, NodeError, Rule};

use super::Sum;

impl TryFrom<Pair<'_, Rule>> for Sum {
    type Error = NodeError;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::sum => Ok(Sum::new(
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

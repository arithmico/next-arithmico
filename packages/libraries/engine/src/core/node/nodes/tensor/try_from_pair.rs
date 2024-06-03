use pest::iterators::Pair;

use crate::core::{
    node::{Node, NodeError},
    parse::Rule,
};

use super::Tensor;

impl TryFrom<Pair<'_, Rule>> for Tensor {
    type Error = NodeError;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::vector => Ok(Tensor::new(
                pair.into_inner()
                    .map(|item| Node::try_from(item))
                    .collect::<Result<Vec<_>, _>>()?,
            )),
            _ => Err(NodeError::ParsingError(String::from(
                "failed to convert parse tree to syntax tree",
            ))),
        }
    }
}

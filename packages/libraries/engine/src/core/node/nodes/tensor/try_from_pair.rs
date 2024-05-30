use pest::iterators::Pair;

use crate::core::node::{
    errors::TransformParseTreeError, parse::parser::Rule, Node,
};

use super::Tensor;

impl TryFrom<Pair<'_, Rule>> for Tensor {
    type Error = TransformParseTreeError;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::vector => Ok(Tensor::new(
                pair.into_inner()
                    .map(|item| Node::try_from(item))
                    .collect::<Result<Vec<_>, _>>()?,
            )),
            _ => Err(TransformParseTreeError::ConversionFailed),
        }
    }
}

use pest::iterators::Pair;

use crate::core::{
    node::{errors::TransformParseTreeError, Node},
    parse::Rule,
};

use super::Negate;

impl TryFrom<Pair<'_, Rule>> for Negate {
    type Error = TransformParseTreeError;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::negate => Ok(Negate::new(
                Node::try_from(pair.into_inner().next().unwrap()).unwrap(),
            )),
            _ => Err(TransformParseTreeError::ConversionFailed),
        }
    }
}

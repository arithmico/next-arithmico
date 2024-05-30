use pest::iterators::Pair;

use crate::core::{
    node::{errors::TransformParseTreeError, Node},
    parse::Rule,
};

use super::And;

impl TryFrom<Pair<'_, Rule>> for And {
    type Error = TransformParseTreeError;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::and => {
                let items = pair
                    .into_inner()
                    .map(|item| Node::try_from(item))
                    .collect::<Result<Vec<Node>, _>>()?;

                Ok(Self::new(items))
            }
            _ => Err(TransformParseTreeError::ConversionFailed),
        }
    }
}

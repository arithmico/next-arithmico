use pest::iterators::Pair;

use crate::core::node::{NodeError, Rule};

use super::Boolean;

impl TryFrom<Pair<'_, Rule>> for Boolean {
    type Error = NodeError;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::boolean => Ok(Boolean::new(pair.as_str() == "true")),
            _ => Err(NodeError::ParsingError(String::from(
                "failed to convert parse tree to syntax tree",
            ))),
        }
    }
}

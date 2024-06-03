use pest::iterators::Pair;

use crate::core::node::{NodeError, Rule};

use super::Number;

impl TryFrom<Pair<'_, Rule>> for Number {
    type Error = NodeError;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::number => Ok(Number::new(pair.as_str().parse().or(Err(
                NodeError::ParsingError(String::from(
                    "failed to convert parse tree to syntax tree",
                )),
            ))?)),
            _ => Err(NodeError::ParsingError(String::from(
                "failed to convert parse tree to syntax tree",
            ))),
        }
    }
}

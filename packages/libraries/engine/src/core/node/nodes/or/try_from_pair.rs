use pest::iterators::Pair;

use crate::core::{
    node::{Node, NodeError},
    parse::Rule,
};

use super::Or;

impl TryFrom<Pair<'_, Rule>> for Or {
    type Error = NodeError;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::or => {
                let items = pair
                    .into_inner()
                    .map(|item| Node::try_from(item))
                    .collect::<Result<Vec<Node>, _>>()?;

                Ok(Self::new(items))
            }
            _ => Err(NodeError::ParsingError(String::from(
                "failed to convert parse tree to syntax tree",
            ))),
        }
    }
}

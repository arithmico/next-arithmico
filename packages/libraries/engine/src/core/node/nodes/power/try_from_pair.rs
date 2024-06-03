use pest::iterators::Pair;

use crate::core::{
    node::{Node, NodeError},
    parse::Rule,
};

use super::Power;

impl TryFrom<Pair<'_, Rule>> for Power {
    type Error = NodeError;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::power => {
                let mut inner_pairs = pair.into_inner();
                let base = Node::try_from(inner_pairs.next().unwrap()).unwrap();
                let exponent =
                    Node::try_from(inner_pairs.next().unwrap()).unwrap();
                Ok(Power::new(base, exponent))
            }
            _ => Err(NodeError::ParsingError(String::from(
                "failed to convert parse tree to syntax tree",
            ))),
        }
    }
}

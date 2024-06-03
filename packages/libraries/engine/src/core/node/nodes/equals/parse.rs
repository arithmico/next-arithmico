use pest::iterators::Pair;

use crate::core::node::{Node, NodeError, Rule};

use super::Equals;

impl TryFrom<Pair<'_, Rule>> for Equals {
    type Error = NodeError;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::equals => {
                let mut inner_pairs = pair.into_inner();
                let left = Node::try_from(inner_pairs.next().unwrap()).unwrap();
                let right =
                    Node::try_from(inner_pairs.next().unwrap()).unwrap();
                Ok(Equals::new(left, right))
            }
            _ => Err(NodeError::ParsingError(String::from(
                "failed to convert parse tree to syntax tree",
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::node::*;

    #[test]
    fn parse_equals() {
        assert_eq!(
            Node::parse("a = b").unwrap(),
            Node::from(Equals::new(Symbol::new("a"), Symbol::new("b")))
        )
    }
}

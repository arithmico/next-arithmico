use pest::iterators::Pair;

use crate::core::node::{Node, NodeError, Rule};

use super::Negate;

impl TryFrom<Pair<'_, Rule>> for Negate {
    type Error = NodeError;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::negate => Ok(Negate::new(
                Node::try_from(pair.into_inner().next().unwrap()).unwrap(),
            )),
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
    fn parse_negate() {
        let result = Node::parse("-1").unwrap();
        assert_eq!(result, Negate::new(Number::new(1.0)).into());
    }
}

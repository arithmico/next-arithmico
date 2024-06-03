use pest::iterators::Pair;

use crate::core::node::{Node, NodeError, Rule};

use super::Division;

impl TryFrom<Pair<'_, Rule>> for Division {
    type Error = NodeError;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::division => {
                let mut inner_pairs = pair.into_inner();
                let dividend =
                    Node::try_from(inner_pairs.next().unwrap()).unwrap();
                let divisor =
                    Node::try_from(inner_pairs.next().unwrap()).unwrap();
                Ok(Division::new(dividend, divisor))
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
    fn parse_division() {
        let result = Node::parse("1 / 2").unwrap();
        assert_eq!(
            result,
            Division::new(Number::new(1.0), Number::new(2.0),).into()
        );
    }
}

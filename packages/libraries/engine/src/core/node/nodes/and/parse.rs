use pest::iterators::Pair;

use crate::core::node::{Node, NodeError, Rule};

use super::And;

impl TryFrom<Pair<'_, Rule>> for And {
    type Error = NodeError;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::and => {
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

#[cfg(test)]
mod tests {
    use crate::core::node::*;

    #[test]
    fn parse_and_with_2_values() {
        let result = Node::parse("a & b").unwrap();
        assert_eq!(
            result,
            And::new(vec![Symbol::new("a").into(), Symbol::new("b").into(),])
                .into()
        );
    }

    #[test]
    fn parse_and_with_3_values() {
        let result = Node::parse("a & b & c").unwrap();
        assert_eq!(
            result,
            And::new(vec![
                Symbol::new("a").into(),
                Symbol::new("b").into(),
                Symbol::new("c").into()
            ])
            .into()
        );
    }
}

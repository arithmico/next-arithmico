use pest::iterators::Pair;

use crate::core::node::{Node, NodeError, Rule};

use super::Tensor;

impl TryFrom<Pair<'_, Rule>> for Tensor {
    type Error = NodeError;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::vector => Ok(Tensor::new(
                pair.into_inner()
                    .map(|item| Node::try_from(item))
                    .collect::<Result<Vec<_>, _>>()?,
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
    fn parse_tensor() {
        let result = Node::parse("[1, 2, 3]").unwrap();
        assert_eq!(
            result,
            Tensor::new(vec![
                Number::new(1.0).into(),
                Number::new(2.0).into(),
                Number::new(3.0).into(),
            ])
            .into()
        );
    }
}

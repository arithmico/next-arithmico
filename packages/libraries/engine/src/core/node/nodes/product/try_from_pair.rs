use pest::iterators::Pair;

use crate::core::node::{Node, NodeError, Rule};

use super::Product;

impl TryFrom<Pair<'_, Rule>> for Product {
    type Error = NodeError;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::product => Ok(Product::new(
                pair.into_inner()
                    .map(|item| Node::try_from(item).unwrap())
                    .collect(),
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
    fn parse_product() {
        let result = Node::parse("1 * 2").unwrap();
        assert_eq!(
            result,
            Product::new(vec![
                Number::new(1.0).into(),
                Number::new(2.0).into(),
            ])
            .into()
        );
    }

    #[test]
    fn parse_product_with_inner_sum() {
        let result = Node::parse("(1 + 2) * 3").unwrap();
        assert_eq!(
            result,
            Product::new(vec![
                Sum::new(vec![
                    Number::new(1.0).into(),
                    Number::new(2.0).into(),
                ])
                .into(),
                Number::new(3.0).into(),
            ])
            .into()
        );
    }

    #[test]
    fn parse_product_with_division() {
        let result = Node::parse("1 / 2 * 3").unwrap();
        assert_eq!(
            result,
            Product::new(vec![
                Division::new(Number::new(1.0), Number::new(2.0)).into(),
                Number::new(3.0).into()
            ])
            .into()
        );
    }
}

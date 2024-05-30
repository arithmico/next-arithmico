use pest::iterators::Pair;

use crate::core::node::{
    errors::TransformParseTreeError, parse::parser::Rule, Node,
};

use super::Division;

impl TryFrom<Pair<'_, Rule>> for Division {
    type Error = TransformParseTreeError;

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
            _ => Err(TransformParseTreeError::ConversionFailed),
        }
    }
}

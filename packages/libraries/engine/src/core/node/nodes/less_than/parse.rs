use pest::iterators::Pair;

use crate::{
    core::node::{Node, NodeError, Rule},
    utils::parse_utils::get_next_pair_from_pairs,
};

use super::LessThan;

impl TryFrom<Pair<'_, Rule>> for LessThan {
    type Error = NodeError;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::less_than => {
                let mut inner_pairs = pair.into_inner();
                let left: Node =
                    get_next_pair_from_pairs(&mut inner_pairs)?.try_into()?;
                let right: Node =
                    get_next_pair_from_pairs(&mut inner_pairs)?.try_into()?;

                Ok(LessThan::new(left, right))
            }
            _ => Err(NodeError::ParsingError(
                "failed to convert parse tree to syntax tree".into(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::node::*;

    #[test]
    fn parse_equals() {
        assert_eq!(
            Node::parse("a < b").unwrap(),
            Node::from(LessThan::new(Symbol::new("a"), Symbol::new("b")))
        )
    }
}

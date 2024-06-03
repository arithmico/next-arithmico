use pest::iterators::Pair;

use crate::{
    core::node::{Node, NodeError, Rule, Symbol},
    utils::parse_utils::next_pair_of_rule,
};

use super::Function;

impl TryFrom<Pair<'_, Rule>> for Function {
    type Error = NodeError;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::function => {
                let mut inner_pairs = pair.into_inner();
                let arguments = next_pair_of_rule(
                    &mut inner_pairs,
                    Rule::function_arguments,
                )
                .into_inner()
                .map(|argument| {
                    Symbol::try_from(argument)
                        .and_then(|symbol| Ok(symbol.name))
                })
                .collect::<Result<Vec<_>, _>>()?;

                let expression = Node::try_from(inner_pairs.next().unwrap())?;
                Ok(Function::new(arguments, expression))
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
    fn parse_function() {
        let result = Node::parse("(x, y) -> x + y").unwrap();
        assert_eq!(
            result,
            Function::new(
                vec!["x".into(), "y".into()],
                Sum::new(vec![
                    Symbol::new("x").into(),
                    Symbol::new("y").into()
                ])
            )
            .into()
        );
    }
}

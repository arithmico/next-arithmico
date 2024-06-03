use pest::iterators::Pair;

use crate::{
    core::node::{Function, Node, NodeError, Rule},
    utils::parse_utils::next_pair_of_rule,
};

use super::Definition;

impl TryFrom<Pair<'_, Rule>> for Definition {
    type Error = NodeError;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::function_definition => {
                let mut inner_pairs = pair.into_inner();
                let symbol = String::from(
                    next_pair_of_rule(
                        &mut inner_pairs,
                        Rule::function_definition_target,
                    )
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str(),
                );
                let arguments = next_pair_of_rule(
                    &mut inner_pairs,
                    Rule::function_definition_arguments,
                )
                .into_inner()
                .map(|argument| String::from(argument.as_str()))
                .collect();
                let expression = Node::try_from(inner_pairs.next().unwrap())?;

                Ok(Definition::new(
                    symbol,
                    Function::new(arguments, expression),
                ))
            }
            Rule::symbol_definition => {
                let mut inner_pairs = pair.into_inner();
                let symbol = String::from(inner_pairs.next().unwrap().as_str());
                let expression = Node::try_from(inner_pairs.next().unwrap())?;
                Ok(Definition::new(symbol, expression))
            }
            _ => Err(NodeError::ParsingError(String::from(
                "failed to convert parse tree to syntax tree",
            ))),
        }
    }
}

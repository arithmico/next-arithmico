use pest::iterators::Pair;

use crate::core::node::{
    errors::TransformParseTreeError,
    parse::{parser::Rule, transform::next_pair_of_rule},
    Node,
};

use super::FunctionCall;

impl TryFrom<Pair<'_, Rule>> for FunctionCall {
    type Error = TransformParseTreeError;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::function_call => {
                let mut inner_pairs = pair.into_inner();
                let target = Node::try_from(
                    next_pair_of_rule(
                        &mut inner_pairs,
                        Rule::function_call_target,
                    )
                    .into_inner()
                    .next()
                    .unwrap(),
                )?;
                let arguments = next_pair_of_rule(
                    &mut inner_pairs,
                    Rule::function_call_arguments,
                )
                .into_inner()
                .map(|argument| Node::try_from(argument))
                .collect::<Result<Vec<_>, _>>()?;
                Ok(FunctionCall::new(target, arguments))
            }
            _ => Err(TransformParseTreeError::ConversionFailed),
        }
    }
}

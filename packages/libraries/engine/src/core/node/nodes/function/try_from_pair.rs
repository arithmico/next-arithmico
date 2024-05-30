use pest::iterators::Pair;

use crate::core::node::{
    errors::TransformParseTreeError,
    parse::{parser::Rule, transform::next_pair_of_rule},
    Node, Symbol,
};

use super::Function;

impl TryFrom<Pair<'_, Rule>> for Function {
    type Error = TransformParseTreeError;

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
            _ => Err(TransformParseTreeError::ConversionFailed),
        }
    }
}

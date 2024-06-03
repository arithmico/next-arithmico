use node::Rule;
use pest::iterators::Pair;

use crate::core::node::*;

use super::Node;

impl TryFrom<Pair<'_, Rule>> for Node {
    type Error = NodeError;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::statement => {
                Node::try_from(pair.into_inner().next().unwrap())
            }
            Rule::number => Ok(Number::try_from(pair)?.into()),
            Rule::boolean => Ok(Boolean::try_from(pair)?.into()),
            Rule::symbol => Ok(Symbol::try_from(pair)?.into()),
            Rule::sum => Ok(Sum::try_from(pair)?.into()),
            Rule::negate => Ok(Negate::try_from(pair)?.into()),
            Rule::product => Ok(Product::try_from(pair)?.into()),
            Rule::division => Ok(Division::try_from(pair)?.into()),
            Rule::power => Ok(Power::try_from(pair)?.into()),
            Rule::vector => Ok(Tensor::try_from(pair)?.into()),
            Rule::function_call => Ok(FunctionCall::try_from(pair)?.into()),
            Rule::function => Ok(Function::try_from(pair)?.into()),
            Rule::function_definition | Rule::symbol_definition => {
                Ok(Definition::try_from(pair)?.into())
            }
            Rule::and => Ok(And::try_from(pair)?.into()),
            Rule::or => Ok(Or::try_from(pair)?.into()),
            Rule::equals => Ok(Equals::try_from(pair)?.into()),
            _ => Err(NodeError::ParsingError(String::from(
                "failed to convert parse tree to syntax tree",
            ))),
        }
    }
}

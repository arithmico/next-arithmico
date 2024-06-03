use pest::iterators::Pair;

use crate::{
    core::{
        node::{Node, NodeError, Symbol},
        parse::Rule,
    },
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
    use crate::utils::test_utils::serialization_test;

    #[test]
    fn serialize_function_1() {
        serialization_test("() -> 2", "() -> 2");
    }

    #[test]
    fn serialize_function_2() {
        serialization_test("(x) -> x^2", "(x) -> x^2");
    }
}

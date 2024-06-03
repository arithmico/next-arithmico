use pest::iterators::Pair;

use crate::core::{
    node::{NodeError, Symbol},
    parse::Rule,
};

impl TryFrom<Pair<'_, Rule>> for Symbol {
    type Error = NodeError;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::symbol => Ok(Symbol::new(pair.as_str().to_string()).into()),
            _ => Err(NodeError::ParsingError(String::from(
                "failed to convert parse tree to syntax tree",
            ))),
        }
    }
}

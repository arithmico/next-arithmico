use pest::iterators::Pair;

use crate::core::{
    node::{errors::TransformParseTreeError, Symbol},
    parse::Rule,
};

impl TryFrom<Pair<'_, Rule>> for Symbol {
    type Error = TransformParseTreeError;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::symbol => Ok(Symbol::new(pair.as_str().to_string()).into()),
            _ => Err(TransformParseTreeError::ConversionFailed),
        }
    }
}

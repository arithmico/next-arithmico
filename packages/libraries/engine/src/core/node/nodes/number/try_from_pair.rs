use pest::iterators::Pair;

use crate::core::{node::errors::TransformParseTreeError, parse::Rule};

use super::Number;

impl TryFrom<Pair<'_, Rule>> for Number {
    type Error = TransformParseTreeError;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::number => Ok(Number::new(
                pair.as_str()
                    .parse()
                    .or(Err(TransformParseTreeError::ConversionFailed))?,
            )),
            _ => Err(TransformParseTreeError::ConversionFailed),
        }
    }
}

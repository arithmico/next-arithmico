use pest::iterators::{Pair, Pairs};

use crate::core::node::{NodeError, Rule};

pub fn next_pair_of_rule<'a>(
    pairs: &'a mut Pairs<Rule>,
    rule: Rule,
) -> Pair<'a, Rule> {
    let pair = pairs.next().unwrap();
    if pair.as_rule() != rule {
        panic!("unexpected rule");
    }
    pair
}

pub fn get_next_pair_from_pairs<'a>(
    pairs: &'a mut Pairs<Rule>,
) -> Result<Pair<'a, Rule>, NodeError> {
    pairs
        .next()
        .ok_or(NodeError::ParsingError("failed to get next pair".into()))
}

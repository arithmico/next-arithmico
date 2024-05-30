use pest::iterators::{Pair, Pairs};

use crate::core::parse::Rule;

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

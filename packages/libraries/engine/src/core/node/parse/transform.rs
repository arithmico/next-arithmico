use crate::core::node::{
    And, Boolean, Definition, Division, Function, FunctionCall, Negate, Node,
    Number, Power, Product, Sum, Symbol, Tensor,
};

use super::parser::Rule;
use pest::iterators::{Pair, Pairs};

pub fn transform(pair: Pair<Rule>) -> Node {
    match pair.as_rule() {
        Rule::statement => transform(pair.into_inner().next().unwrap()),
        Rule::number => Number::new(pair.as_str().parse().unwrap()).into(),
        Rule::boolean => Boolean::new(pair.as_str() == "true").into(),
        Rule::symbol => Symbol::new(pair.as_str().to_string()).into(),
        Rule::sum => {
            Sum::new(pair.into_inner().map(|item| transform(item)).collect())
                .into()
        }
        Rule::negate => {
            Negate::new(transform(pair.into_inner().next().unwrap())).into()
        }
        Rule::product => Product::new(
            pair.into_inner().map(|item| transform(item)).collect(),
        )
        .into(),
        Rule::division => {
            let mut inner_pairs = pair.into_inner();
            let dividend = transform(inner_pairs.next().unwrap());
            let divisor = transform(inner_pairs.next().unwrap());
            Division::new(dividend, divisor).into()
        }
        Rule::power => {
            let mut inner_pairs = pair.into_inner();
            let base = transform(inner_pairs.next().unwrap());
            let exponent = transform(inner_pairs.next().unwrap());
            Power::new(base, exponent).into()
        }
        Rule::vector => {
            Tensor::new(pair.into_inner().map(|item| transform(item)).collect())
                .into()
        }
        Rule::function_call => {
            let mut inner_pairs = pair.into_inner();
            let target = transform(
                next_pair_of_rule(&mut inner_pairs, Rule::function_call_target)
                    .into_inner()
                    .next()
                    .unwrap(),
            );
            let arguments = next_pair_of_rule(
                &mut inner_pairs,
                Rule::function_call_arguments,
            )
            .into_inner()
            .map(|argument| transform(argument))
            .collect();
            FunctionCall::new(target, arguments).into()
        }
        Rule::function => {
            let mut inner_pairs = pair.into_inner();
            let arguments =
                next_pair_of_rule(&mut inner_pairs, Rule::function_arguments)
                    .into_inner()
                    .map(|argument| match argument.as_rule() {
                        Rule::symbol => String::from(argument.as_str()),
                        _ => unreachable!(),
                    })
                    .collect();

            let expression = transform(inner_pairs.next().unwrap());
            Function::new(arguments, expression).into()
        }
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
            let expression = transform(inner_pairs.next().unwrap());

            Definition::new(symbol, Function::new(arguments, expression)).into()
        }
        Rule::symbol_definition => {
            let mut inner_pairs = pair.into_inner();
            let symbol = String::from(inner_pairs.next().unwrap().as_str());
            let expression = transform(inner_pairs.next().unwrap());
            Definition::new(symbol, expression).into()
        }
        Rule::and => {
            And::new(pair.into_inner().map(|item| transform(item)).collect())
                .into()
        }
        _ => unreachable!(),
    }
}

fn next_pair_of_rule<'a>(
    pairs: &'a mut Pairs<Rule>,
    rule: Rule,
) -> Pair<'a, Rule> {
    let pair = pairs.next().unwrap();
    if pair.as_rule() != rule {
        panic!("unexpected rule");
    }
    pair
}

#[cfg(test)]
mod tests {
    use super::super::parse::parse_statement;
    use super::*;

    #[test]
    fn transform_float() {
        let result = parse_statement("2.1").unwrap();
        assert_eq!(result, Number::new(2.1).into());
    }

    #[test]
    fn transform_true() {
        let result = parse_statement("true").unwrap();
        assert_eq!(result, Boolean::new(true).into());
    }

    #[test]
    fn transform_false() {
        let result = parse_statement("false").unwrap();
        assert_eq!(result, Boolean::new(false).into());
    }

    #[test]
    fn transform_symbol() {
        let result = parse_statement("hello").unwrap();
        assert_eq!(result, Symbol::new("hello").into());
    }

    #[test]
    fn transform_negate() {
        let result = parse_statement("-1").unwrap();
        assert_eq!(result, Negate::new(Number::new(1.0)).into());
    }

    #[test]
    fn transform_sum() {
        let result = parse_statement("1 + 2 + 3").unwrap();
        assert_eq!(
            result,
            Sum::new(vec![
                Number::new(1.0).into(),
                Number::new(2.0).into(),
                Number::new(3.0).into(),
            ])
            .into()
        );
    }

    #[test]
    fn transform_sum_with_negate() {
        let result = parse_statement("1 + 2 - 3").unwrap();
        assert_eq!(
            result,
            Sum::new(vec![
                Number::new(1.0).into(),
                Number::new(2.0).into(),
                Negate::new(Number::new(3.0)).into()
            ])
            .into()
        );
    }

    #[test]
    fn transform_product() {
        let result = parse_statement("1 * 2").unwrap();
        assert_eq!(
            result,
            Product::new(vec![
                Number::new(1.0).into(),
                Number::new(2.0).into(),
            ])
            .into()
        );
    }

    #[test]
    fn transform_product_with_inner_sum() {
        let result = parse_statement("(1 + 2) * 3").unwrap();
        assert_eq!(
            result,
            Product::new(vec![
                Sum::new(vec![
                    Number::new(1.0).into(),
                    Number::new(2.0).into(),
                ])
                .into(),
                Number::new(3.0).into(),
            ])
            .into()
        );
    }

    #[test]
    fn transform_division() {
        let result = parse_statement("1 / 2").unwrap();
        assert_eq!(
            result,
            Division::new(Number::new(1.0), Number::new(2.0),).into()
        );
    }

    #[test]
    fn transform_product_with_division() {
        let result = parse_statement("1 / 2 * 3").unwrap();
        assert_eq!(
            result,
            Product::new(vec![
                Division::new(Number::new(1.0), Number::new(2.0)).into(),
                Number::new(3.0).into()
            ])
            .into()
        );
    }

    #[test]
    fn transform_power() {
        let result = parse_statement("2 ^ 3").unwrap();
        assert_eq!(
            result,
            Power::new(Number::new(2.0), Number::new(3.0)).into()
        );
    }

    #[test]
    fn transform_tensor() {
        let result = parse_statement("[1, 2, 3]").unwrap();
        assert_eq!(
            result,
            Tensor::new(vec![
                Number::new(1.0).into(),
                Number::new(2.0).into(),
                Number::new(3.0).into(),
            ])
            .into()
        );
    }

    #[test]
    fn transform_and_with_2_values() {
        let result = parse_statement("a & b").unwrap();
        assert_eq!(
            result,
            And::new(vec![Symbol::new("a").into(), Symbol::new("b").into(),])
                .into()
        );
    }

    #[test]
    fn transform_and_with_3_values() {
        let result = parse_statement("a & b & c").unwrap();
        assert_eq!(
            result,
            And::new(vec![
                Symbol::new("a").into(),
                Symbol::new("b").into(),
                Symbol::new("c").into()
            ])
            .into()
        );
    }

    #[test]
    fn transform_function_call() {
        let result = parse_statement("f(1, 2)").unwrap();
        assert_eq!(
            result,
            FunctionCall::new(
                Symbol::new("f"),
                vec![Number::new(1.0).into(), Number::new(2.0).into()]
            )
            .into()
        );
    }

    #[test]
    fn transform_function() {
        let result = parse_statement("(x, y) -> x + y").unwrap();
        assert_eq!(
            result,
            Function::new(
                vec!["x".into(), "y".into()],
                Sum::new(vec![
                    Symbol::new("x").into(),
                    Symbol::new("y").into()
                ])
            )
            .into()
        );
    }

    #[test]
    fn transform_function_definition() {
        let result = parse_statement("f(x, y) := x + y").unwrap();
        assert_eq!(
            result,
            Node::from(Definition::new(
                "f",
                Function::new(
                    vec!["x".into(), "y".into()],
                    Sum::new(vec![
                        Symbol::new("x").into(),
                        Symbol::new("y").into()
                    ])
                )
            ))
        );
    }

    #[test]
    fn transform_symbol_definition() {
        let result = parse_statement("x := 2").unwrap();
        assert_eq!(result, Node::from(Definition::new("x", Number::new(2.0))));
    }
}

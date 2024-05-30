use pest::{error::Error, Parser};

use crate::core::node::Node;

use super::parser::{ArithmicoParser, Rule};

pub fn parse_statement(input: &str) -> Result<Node, Error<Rule>> {
    let pairs = ArithmicoParser::parse(Rule::statement, input)?
        .next()
        .unwrap();

    Ok(Node::try_from(pairs).unwrap())
}

#[cfg(test)]
mod tests {
    use crate::core::node::parse::parse::parse_statement;
    use crate::core::node::*;

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
    fn transform_or_with_2_values() {
        let result = parse_statement("a | b").unwrap();
        assert_eq!(
            result,
            Or::new(vec![Symbol::new("a").into(), Symbol::new("b").into(),])
                .into()
        );
    }

    #[test]
    fn transform_or_with_3_values() {
        let result = parse_statement("a | b | c").unwrap();
        assert_eq!(
            result,
            Or::new(vec![
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

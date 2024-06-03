use crate::core::node::{Node, NodeError};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ArithmicoParser;
use pest::Parser;

impl Node {
    pub fn parse(input: &str) -> Result<Self, NodeError> {
        let pairs = ArithmicoParser::parse(Rule::statement, input)?
            .next()
            .unwrap();

        Ok(Self::try_from(pairs).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::core::node::*;

    #[test]
    fn transform_float() {
        let result = Node::parse("2.1").unwrap();
        assert_eq!(result, Number::new(2.1).into());
    }

    #[test]
    fn transform_true() {
        let result = Node::parse("true").unwrap();
        assert_eq!(result, Boolean::new(true).into());
    }

    #[test]
    fn transform_false() {
        let result = Node::parse("false").unwrap();
        assert_eq!(result, Boolean::new(false).into());
    }

    #[test]
    fn transform_symbol() {
        let result = Node::parse("hello").unwrap();
        assert_eq!(result, Symbol::new("hello").into());
    }

    #[test]
    fn transform_negate() {
        let result = Node::parse("-1").unwrap();
        assert_eq!(result, Negate::new(Number::new(1.0)).into());
    }

    #[test]
    fn transform_sum() {
        let result = Node::parse("1 + 2 + 3").unwrap();
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
        let result = Node::parse("1 + 2 - 3").unwrap();
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
        let result = Node::parse("1 * 2").unwrap();
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
        let result = Node::parse("(1 + 2) * 3").unwrap();
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
        let result = Node::parse("1 / 2").unwrap();
        assert_eq!(
            result,
            Division::new(Number::new(1.0), Number::new(2.0),).into()
        );
    }

    #[test]
    fn transform_product_with_division() {
        let result = Node::parse("1 / 2 * 3").unwrap();
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
        let result = Node::parse("2 ^ 3").unwrap();
        assert_eq!(
            result,
            Power::new(Number::new(2.0), Number::new(3.0)).into()
        );
    }

    #[test]
    fn transform_tensor() {
        let result = Node::parse("[1, 2, 3]").unwrap();
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
        let result = Node::parse("a & b").unwrap();
        assert_eq!(
            result,
            And::new(vec![Symbol::new("a").into(), Symbol::new("b").into(),])
                .into()
        );
    }

    #[test]
    fn transform_and_with_3_values() {
        let result = Node::parse("a & b & c").unwrap();
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
        let result = Node::parse("a | b").unwrap();
        assert_eq!(
            result,
            Or::new(vec![Symbol::new("a").into(), Symbol::new("b").into(),])
                .into()
        );
    }

    #[test]
    fn transform_or_with_3_values() {
        let result = Node::parse("a | b | c").unwrap();
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
        let result = Node::parse("f(1, 2)").unwrap();
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
        let result = Node::parse("(x, y) -> x + y").unwrap();
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
        let result = Node::parse("f(x, y) := x + y").unwrap();
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
        let result = Node::parse("x := 2").unwrap();
        assert_eq!(result, Node::from(Definition::new("x", Number::new(2.0))));
    }
}

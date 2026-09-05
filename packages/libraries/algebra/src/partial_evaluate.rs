use std::ops::ControlFlow;

use evaluator::{Error, EvaluateNode, Options};
use node::{And, Node, NodeVisitorMut, Or, Product, Sum};

use crate::{IsBound, sealed::Sealed};

#[allow(private_bounds)]
pub trait PartialEvaluate: Sealed {
    fn partial_evaluate<'a>(
        &'a mut self,
        options: Options<'a>,
    ) -> Result<(), Error>;
}

impl PartialEvaluate for Node {
    fn partial_evaluate<'a>(
        &'a mut self,
        options: Options<'a>,
    ) -> Result<(), Error> {
        let mut visitor = PartialEvaluateVisitor::new(options);
        match self.visit_post_order_mut(&mut visitor) {
            ControlFlow::Continue(_) => Ok(()),
            ControlFlow::Break(err) => Err(err),
        }
    }
}

struct PartialEvaluateVisitor<'a> {
    options: Options<'a>,
}

impl<'a> PartialEvaluateVisitor<'a> {
    pub fn new(options: Options<'a>) -> Self {
        Self { options }
    }
}

impl<'a> NodeVisitorMut for PartialEvaluateVisitor<'a> {
    type Break = Error;

    fn visit(&mut self, node: &mut Node) -> ControlFlow<Self::Break, ()> {
        match partial_evaluate(node, self.options) {
            Ok(_) => ControlFlow::Continue(()),
            Err(err) => ControlFlow::Break(err),
        }
    }
}

fn partial_evaluate<'a>(
    node: &'a mut Node,
    options: Options<'a>,
) -> Result<(), Error> {
    let known_symbols = options.names();
    if node.is_bound(&known_symbols) {
        *node = node.evaluate(options)?;
        return Ok(());
    }

    match node {
        Node::Sum(inner) => {
            // combine children if possible
            let mut pos = 0;
            while let Some(left) = inner.elements.get(pos)
                && let Some(right) = inner.elements.get(pos + 1)
            {
                match (
                    left.is_bound(&known_symbols),
                    right.is_bound(&known_symbols),
                ) {
                    (true, true) => {
                        let partial_sum =
                            Sum::new(vec![left.clone(), right.clone()])
                                .evaluate(options)?;
                        inner.elements[pos] = partial_sum;
                        inner.elements.remove(pos + 1);
                    }
                    _ => {
                        pos += 1;
                    }
                }
            }

            // replace sum if it only has one element
            if inner.elements.len() == 1
                && let Some(element) = inner.elements.pop()
            {
                *node = element;
            }
        }
        Node::Product(inner) => {
            // combine children if possible
            let mut pos = 0;
            while let Some(left) = inner.elements.get(pos)
                && let Some(right) = inner.elements.get(pos + 1)
            {
                match (
                    left.is_bound(&known_symbols),
                    right.is_bound(&known_symbols),
                ) {
                    (true, true) => {
                        let partial_sum = Product::new_node(vec![
                            left.clone(),
                            right.clone(),
                        ])
                        .evaluate(options)?;
                        inner.elements[pos] = partial_sum;
                        inner.elements.remove(pos + 1);
                    }
                    _ => {
                        pos += 1;
                    }
                }
            }

            // replace product if it only has one element
            if inner.elements.len() == 1
                && let Some(element) = inner.elements.pop()
            {
                *node = element;
            }
        }
        Node::And(inner) => {
            // combine children if possible
            let mut pos = 0;
            while let Some(left) = inner.elements.get(pos)
                && let Some(right) = inner.elements.get(pos + 1)
            {
                match (
                    left.is_bound(&known_symbols),
                    right.is_bound(&known_symbols),
                ) {
                    (true, true) => {
                        let partial_sum =
                            And::new(vec![left.clone(), right.clone()])
                                .evaluate(options)?;
                        inner.elements[pos] = partial_sum;
                        inner.elements.remove(pos + 1);
                    }
                    _ => {
                        pos += 1;
                    }
                }
            }

            // replace and if it only has one element
            if inner.elements.len() == 1
                && let Some(element) = inner.elements.pop()
            {
                *node = element;
            }
        }
        Node::Or(inner) => {
            // combine children if possible
            let mut pos = 0;
            while let Some(left) = inner.elements.get(pos)
                && let Some(right) = inner.elements.get(pos + 1)
            {
                match (
                    left.is_bound(&known_symbols),
                    right.is_bound(&known_symbols),
                ) {
                    (true, true) => {
                        let partial_sum =
                            Or::new(vec![left.clone(), right.clone()])
                                .evaluate(options)?;
                        inner.elements[pos] = partial_sum;
                        inner.elements.remove(pos + 1);
                    }
                    _ => {
                        pos += 1;
                    }
                }
            }

            // replace or if it only has one element
            if inner.elements.len() == 1
                && let Some(element) = inner.elements.pop()
            {
                *node = element;
            }
        }
        _ => (),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use common::AngleUnit;
    use evaluator::{Api, Stack};
    use node::{Boolean, Number, Symbol};

    use super::*;

    #[test]
    fn sum_evaluate() {
        let api = Api::default();
        let stack = Stack::new();
        let options = Options::new(&stack, &api, AngleUnit::default());
        let mut output =
            Sum::new(vec![Number::new_node(1.0), Number::new_node(2.0)]);
        output.partial_evaluate(options).unwrap();
        assert_eq!(output, Number::new_node(3.0));
    }

    #[test]
    fn sum_one_child() {
        let api = Api::default();
        let stack = Stack::new();
        let options = Options::new(&stack, &api, AngleUnit::default());
        let mut output = Sum::new(vec![
            Sum::new(vec![Number::new_node(1.0), Number::new_node(2.0)]),
            Symbol::new("x"),
        ]);
        output.partial_evaluate(options).unwrap();
        assert_eq!(
            output,
            Sum::new(vec![Number::new_node(3.0), Symbol::new("x")])
        );
    }

    #[test]
    fn sum_combine_children() {
        let api = Api::default();
        let stack = Stack::new();
        let options = Options::new(&stack, &api, AngleUnit::default());
        let mut output = Sum::new(vec![
            Number::new_node(1.0),
            Number::new_node(2.0),
            Symbol::new("x"),
        ]);
        output.partial_evaluate(options).unwrap();
        assert_eq!(
            output,
            Sum::new(vec![Number::new_node(3.0), Symbol::new("x")])
        );
    }

    #[test]
    fn product_evaluate() {
        let api = Api::default();
        let stack = Stack::new();
        let options = Options::new(&stack, &api, AngleUnit::default());
        let mut output = Product::new_node(vec![
            Number::new_node(1.0),
            Number::new_node(2.0),
        ]);
        output.partial_evaluate(options).unwrap();
        assert_eq!(output, Number::new_node(2.0));
    }

    #[test]
    fn product_one_child() {
        let api = Api::default();
        let stack = Stack::new();
        let options = Options::new(&stack, &api, AngleUnit::default());
        let mut output = Product::new_node(vec![
            Sum::new(vec![Number::new_node(1.0), Number::new_node(2.0)]),
            Symbol::new("x"),
        ]);
        output.partial_evaluate(options).unwrap();
        assert_eq!(
            output,
            Product::new_node(vec![Number::new_node(3.0), Symbol::new("x")])
        );
    }

    #[test]
    fn product_combine_children() {
        let api = Api::default();
        let stack = Stack::new();
        let options = Options::new(&stack, &api, AngleUnit::default());
        let mut output = Product::new_node(vec![
            Number::new_node(1.0),
            Number::new_node(2.0),
            Symbol::new("x"),
        ]);
        output.partial_evaluate(options).unwrap();
        assert_eq!(
            output,
            Product::new_node(vec![Number::new_node(2.0), Symbol::new("x")])
        );
    }

    #[test]
    fn and_evaluate() {
        let api = Api::default();
        let stack = Stack::new();
        let options = Options::new(&stack, &api, AngleUnit::default());
        let mut output = And::new(vec![Boolean::new(true), Boolean::new(true)]);
        output.partial_evaluate(options).unwrap();
        assert_eq!(output, Boolean::new(true));
    }

    #[test]
    fn and_one_child() {
        let api = Api::default();
        let stack = Stack::new();
        let options = Options::new(&stack, &api, AngleUnit::default());
        let mut output = And::new(vec![
            Or::new(vec![Boolean::new(false), Boolean::new(true)]),
            Symbol::new("x"),
        ]);
        output.partial_evaluate(options).unwrap();
        assert_eq!(
            output,
            And::new(vec![Boolean::new(true), Symbol::new("x")])
        );
    }

    #[test]
    fn and_combine_children() {
        let api = Api::default();
        let stack = Stack::new();
        let options = Options::new(&stack, &api, AngleUnit::default());
        let mut output = And::new(vec![
            Boolean::new(true),
            Boolean::new(false),
            Symbol::new("x"),
        ]);
        output.partial_evaluate(options).unwrap();
        assert_eq!(
            output,
            And::new(vec![Boolean::new(false), Symbol::new("x")])
        );
    }

    #[test]
    fn or_evaluate() {
        let api = Api::default();
        let stack = Stack::new();
        let options = Options::new(&stack, &api, AngleUnit::default());
        let mut output = Or::new(vec![Boolean::new(true), Boolean::new(true)]);
        output.partial_evaluate(options).unwrap();
        assert_eq!(output, Boolean::new(true));
    }

    #[test]
    fn or_one_child() {
        let api = Api::default();
        let stack = Stack::new();
        let options = Options::new(&stack, &api, AngleUnit::default());
        let mut output = Or::new(vec![
            And::new(vec![Boolean::new(false), Boolean::new(true)]),
            Symbol::new("x"),
        ]);
        output.partial_evaluate(options).unwrap();
        assert_eq!(
            output,
            Or::new(vec![Boolean::new(false), Symbol::new("x")])
        );
    }

    #[test]
    fn or_combine_children() {
        let api = Api::default();
        let stack = Stack::new();
        let options = Options::new(&stack, &api, AngleUnit::default());
        let mut output = Or::new(vec![
            Boolean::new(true),
            Boolean::new(false),
            Symbol::new("x"),
        ]);
        output.partial_evaluate(options).unwrap();
        assert_eq!(output, Or::new(vec![Boolean::new(true), Symbol::new("x")]));
    }
}

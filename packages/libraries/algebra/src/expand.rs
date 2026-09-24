use std::ops::ControlFlow;

use evaluator::{Error, Options};
use node::{Node, NodeVisitorMut};
use trace::{Tracable, TracableMut};

use crate::{PartialEvaluate, sealed::Sealed};

#[allow(private_bounds)]
pub trait ExpandNode: Sealed {
    fn expand(&mut self, options: Options) -> Result<(), Error>;
}

impl ExpandNode for Node {
    fn expand(&mut self, options: Options) -> Result<(), Error> {
        self.partial_evaluate(options)?;
        let mut visitor = ExpandVisitor::new(options);
        match self.visit_post_order_mut(&mut visitor) {
            ControlFlow::Continue(_) => (),
            ControlFlow::Break(err) => {
                return Err(err.with_optional_new_frame(self.hull()));
            }
        }
        Ok(())
    }
}

struct ExpandVisitor<'a> {
    options: Options<'a>,
}

impl<'a> ExpandVisitor<'a> {
    pub fn new(options: Options<'a>) -> Self {
        Self { options }
    }
}

impl<'a> NodeVisitorMut for ExpandVisitor<'a> {
    type Break = Error;

    fn visit(&mut self, node: &mut Node) -> ControlFlow<Self::Break, ()> {
        match expand(node, self.options) {
            Ok(_) => ControlFlow::Continue(()),
            Err(err) => ControlFlow::Break(err),
        }
    }
}

fn expand<'a>(node: &'a mut Node, _options: Options<'a>) -> Result<(), Error> {
    match node {
        Node::Sum(inner) => {
            if !inner.elements.iter().any(|e| matches!(e, Node::Sum(_))) {
                return Ok(());
            }

            // flatten nested sums
            let mut new_len = inner.elements.len();
            inner.elements.iter().for_each(|e| {
                if let Node::Sum(sum) = e {
                    new_len += sum.elements.len().saturating_sub(1);
                }
            });

            let mut elements = Vec::with_capacity(new_len);
            std::mem::swap(&mut inner.elements, &mut elements);

            for element in elements {
                match element {
                    Node::Sum(sum) => {
                        inner.elements.extend(sum.elements);
                    }
                    other => inner.elements.push(other),
                }
            }
        }
        Node::Product(inner) => {
            // flatten nested products
            if inner.elements.iter().any(|e| matches!(e, Node::Product(_))) {
                let mut new_len = inner.elements.len();
                inner.elements.iter().for_each(|e| {
                    if let Node::Sum(sum) = e {
                        new_len += sum.elements.len().saturating_sub(1);
                    }
                });

                let mut elements = Vec::with_capacity(new_len);
                std::mem::swap(&mut inner.elements, &mut elements);

                for element in elements {
                    match element {
                        Node::Product(sum) => {
                            inner.elements.extend(sum.elements);
                        }
                        other => inner.elements.push(other),
                    }
                }
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
    use node::{Product, Sum, Symbol};

    use super::*;

    #[test]
    fn sum_flatten() {
        let api = Api::default();
        let stack = Stack::new();
        let options = Options::new(&stack, &api, AngleUnit::default());
        let mut output = Sum::new(vec![
            Sum::new(vec![Symbol::new("a"), Symbol::new("b")]),
            Symbol::new("c"),
        ]);
        output.expand(options).unwrap();
        assert_eq!(
            output,
            Sum::new(vec![
                Symbol::new("a"),
                Symbol::new("b"),
                Symbol::new("c"),
            ])
        )
    }

    #[test]
    fn product_flatten() {
        let api = Api::default();
        let stack = Stack::new();
        let options = Options::new(&stack, &api, AngleUnit::default());
        let mut output = Product::new(vec![
            Product::new(vec![Symbol::new("a"), Symbol::new("b")]),
            Symbol::new("c"),
        ]);
        output.expand(options).unwrap();
        assert_eq!(
            output,
            Product::new(vec![
                Symbol::new("a"),
                Symbol::new("b"),
                Symbol::new("c"),
            ])
        )
    }
}

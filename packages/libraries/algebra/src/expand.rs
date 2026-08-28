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
            // flatten nested sums
            let mut pos = 0;
            while let Some(element) = inner.elements.get(pos) {
                if let Node::Sum(element) = element {
                    inner
                        .elements
                        .reserve(element.elements.len().saturating_sub(1));
                    let Node::Sum(element) = inner.elements.remove(pos) else {
                        // unreachable
                        continue;
                    };

                    for element in element.elements {
                        inner.elements.insert(pos, element);
                        pos += 1;
                    }
                } else {
                    pos += 1;
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
    use node::{Sum, Symbol};

    use super::*;

    #[test]
    fn sum_flatten() {
        let api = Api::default();
        let stack = Stack::new();
        let options = Options::new(&stack, &api, AngleUnit::default());
        let mut sum = Sum::new(vec![
            Sum::new(vec![Symbol::new("a"), Symbol::new("b")]),
            Symbol::new("c"),
        ]);
        sum.expand(options).unwrap();
        assert_eq!(
            sum,
            Sum::new(vec![
                Symbol::new("a"),
                Symbol::new("b"),
                Symbol::new("c"),
            ])
        )
    }
}

use std::{collections::HashSet, ops::ControlFlow};

use node::{Node, NodeVisitor};

use crate::sealed::Sealed;

struct IsBoundVisitor<'a> {
    known_symbols: &'a HashSet<&'a str>,
}

impl<'a> NodeVisitor<'a> for IsBoundVisitor<'a> {
    type Break = ();

    fn visit(&mut self, node: &'a node::Node) -> ControlFlow<Self::Break, ()> {
        if let Node::Symbol(symbol) = node
            && !self.known_symbols.contains(symbol.name.as_str())
        {
            return ControlFlow::Break(());
        }
        ControlFlow::Continue(())
    }
}

#[allow(private_bounds)]
pub trait IsBound: Sealed {
    /// Determines whether an expression is fully bound.
    ///
    /// An expression is considered fully bound if all of its variables or symbols
    /// are defined within the provided `known_symbols` set. If the expression
    /// contains any free or unknown variables, this evaluates to `false`.
    fn is_bound(&self, known_symbols: &HashSet<&str>) -> bool;
}

impl IsBound for Node {
    fn is_bound(&self, known_symbols: &HashSet<&str>) -> bool {
        let mut visitor = IsBoundVisitor { known_symbols };
        match self.visit_post_order(&mut visitor) {
            ControlFlow::Continue(_) => true,
            ControlFlow::Break(_) => false,
        }
    }
}

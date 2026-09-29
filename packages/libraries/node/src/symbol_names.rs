use std::{collections::BTreeSet, convert::Infallible, ops::ControlFlow};

use crate::{Node, visitor::NodeVisitor};

struct SymbolNames<'a> {
    names: BTreeSet<&'a str>,
}

impl<'a> SymbolNames<'a> {
    fn new() -> Self {
        Self {
            names: BTreeSet::new(),
        }
    }

    fn collect(self) -> BTreeSet<&'a str> {
        self.names
    }
}

impl<'a> NodeVisitor<'a> for SymbolNames<'a> {
    type Break = Infallible;

    fn visit(
        &mut self,
        node: &'a Node,
    ) -> std::ops::ControlFlow<Self::Break, ()> {
        if let Node::Symbol(symbol) = node {
            self.names.insert(&symbol.name);
        }
        ControlFlow::Continue(())
    }
}

impl Node {
    /// Returns the unique symbol names contained in `node`.
    pub fn get_symbol_names(&self) -> BTreeSet<&str> {
        let mut visitor = SymbolNames::new();
        self.visit_post_order(&mut visitor);
        visitor.collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Number, Product, Sum, Symbol};

    use super::*;

    #[test]
    fn symbol() {
        let symbols = BTreeSet::from_iter(vec!["x"]);
        let node = Symbol::new("x");
        let output = node.get_symbol_names();
        assert_eq!(symbols, output);
    }

    #[test]
    fn sum() {
        let symbols = BTreeSet::from_iter(vec!["x"]);
        let node = Sum::new(vec![Number::new_node(1.0), Symbol::new("x")]);

        let output = node.get_symbol_names();
        assert_eq!(symbols, output);
    }

    #[test]
    fn product() {
        let symbols = BTreeSet::from_iter(vec!["x", "y"]);
        let node = Product::new(vec![
            Number::new_node(1.0),
            Symbol::new("x"),
            Symbol::new("y"),
        ]);

        let output = node.get_symbol_names();
        assert_eq!(symbols, output);
    }
}

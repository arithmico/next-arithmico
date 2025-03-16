use trace::{Tracable, TracableMut, Trace};

use crate::core::Node;

impl TracableMut for Node {
    fn trace_mut(&mut self) -> &mut Trace {
        match self {
            Node::Boolean(node) => &mut node.trace,
            Node::Sum(node) => &mut node.trace,
            Node::Negate(node) => &mut node.trace,
            Node::Product(node) => &mut node.trace,
            Node::Division(node) => &mut node.trace,
            Node::Power(node) => &mut node.trace,
            Node::Tensor(node) => &mut node.trace,
            Node::Number(node) => &mut node.trace,
            Node::Symbol(node) => &mut node.trace,
            Node::Function(node) => &mut node.trace,
            Node::FunctionCall(node) => &mut node.trace,
            Node::And(node) => &mut node.trace,
            Node::Or(node) => &mut node.trace,
            Node::Equals(node) => &mut node.trace,
            Node::LessThan(node) => &mut node.trace,
            Node::LessThanOrEquals(node) => &mut node.trace,
            Node::GreaterThan(node) => &mut node.trace,
            Node::GreaterThanOrEquals(node) => &mut node.trace,
            Node::HostFunction(node) => &mut node.trace,
            Node::Definition(node) => &mut node.trace,
        }
    }
}

impl Tracable for Node {
    fn trace(&self) -> &Trace {
        match self {
            Node::Boolean(node) => &node.trace,
            Node::Sum(node) => &node.trace,
            Node::Negate(node) => &node.trace,
            Node::Product(node) => &node.trace,
            Node::Division(node) => &node.trace,
            Node::Power(node) => &node.trace,
            Node::Tensor(node) => &node.trace,
            Node::Number(node) => &node.trace,
            Node::Symbol(node) => &node.trace,
            Node::Function(node) => &node.trace,
            Node::FunctionCall(node) => &node.trace,
            Node::And(node) => &node.trace,
            Node::Or(node) => &node.trace,
            Node::Equals(node) => &node.trace,
            Node::LessThan(node) => &node.trace,
            Node::LessThanOrEquals(node) => &node.trace,
            Node::GreaterThan(node) => &node.trace,
            Node::GreaterThanOrEquals(node) => &node.trace,
            Node::HostFunction(node) => &node.trace,
            Node::Definition(node) => &node.trace,
        }
    }
}

use crate::core::{ForEachChild, Node};

impl Node {
    pub fn for_each_node<F: Fn(&Node) + Copy>(&self, f: F) {
        self.for_each_child(f);
        f(self);
    }

    pub fn for_each_node_mut<F: Fn(&mut Node) + Copy>(&mut self, f: F) {
        self.for_each_child_mut(f);
        f(self);
    }
}

impl ForEachChild for Node {
    fn for_each_child<F: Fn(&Node) + Copy>(&self, f: F) {
        match self {
            Node::Boolean(node) => node.for_each_child(f),
            Node::Sum(node) => node.for_each_child(f),
            Node::Negate(node) => node.for_each_child(f),
            Node::Product(node) => node.for_each_child(f),
            Node::Division(node) => node.for_each_child(f),
            Node::Power(node) => node.for_each_child(f),
            Node::Tensor(node) => node.for_each_child(f),
            Node::Number(node) => node.for_each_child(f),
            Node::Symbol(node) => node.for_each_child(f),
            Node::Function(node) => node.for_each_child(f),
            Node::FunctionCall(node) => node.for_each_child(f),
            Node::And(node) => node.for_each_child(f),
            Node::Or(node) => node.for_each_child(f),
            Node::Equals(node) => node.for_each_child(f),
            Node::LessThan(node) => node.for_each_child(f),
            Node::LessThanOrEquals(node) => node.for_each_child(f),
            Node::GreaterThan(node) => node.for_each_child(f),
            Node::GreaterThanOrEquals(node) => node.for_each_child(f),
            Node::HostFunction(node) => node.for_each_child(f),
            Node::Definition(node) => node.for_each_child(f),
        }
    }

    fn for_each_child_mut<F: Fn(&mut Node) + Copy>(&mut self, f: F) {
        match self {
            Node::Boolean(node) => node.for_each_child_mut(f),
            Node::Sum(node) => node.for_each_child_mut(f),
            Node::Negate(node) => node.for_each_child_mut(f),
            Node::Product(node) => node.for_each_child_mut(f),
            Node::Division(node) => node.for_each_child_mut(f),
            Node::Power(node) => node.for_each_child_mut(f),
            Node::Tensor(node) => node.for_each_child_mut(f),
            Node::Number(node) => node.for_each_child_mut(f),
            Node::Symbol(node) => node.for_each_child_mut(f),
            Node::Function(node) => node.for_each_child_mut(f),
            Node::FunctionCall(node) => node.for_each_child_mut(f),
            Node::And(node) => node.for_each_child_mut(f),
            Node::Or(node) => node.for_each_child_mut(f),
            Node::Equals(node) => node.for_each_child_mut(f),
            Node::LessThan(node) => node.for_each_child_mut(f),
            Node::LessThanOrEquals(node) => node.for_each_child_mut(f),
            Node::GreaterThan(node) => node.for_each_child_mut(f),
            Node::GreaterThanOrEquals(node) => node.for_each_child_mut(f),
            Node::HostFunction(node) => node.for_each_child_mut(f),
            Node::Definition(node) => node.for_each_child_mut(f),
        }
    }
}

use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Trace {
    pub spans: Vec<Span>,
}

impl Trace {
    pub fn new() -> Self {
        Self { spans: Vec::new() }
    }
}

impl Node {
    pub fn trace(&self) -> &Trace {
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

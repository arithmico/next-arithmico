use crate::Node;

impl Node {
    pub fn node_kind(&self) -> &str {
        match self {
            Node::Boolean(_) => "Boolean",
            Node::Sum(_) => "Sum",
            Node::Negate(_) => "Negate",
            Node::Product(_) => "Product",
            Node::Division(_) => "Division",
            Node::Power(_) => "Power",
            Node::Tensor(_) => "Tensor",
            Node::Number(_) => "Number",
            Node::Symbol(_) => "Symbol",
            Node::Function(_) => "Function",
            Node::FunctionCall(_) => "FunctionCall",
            Node::And(_) => "And",
            Node::Or(_) => "Or",
            Node::Equals(_) => "Equals",
            Node::LessThan(_) => "LessThan",
            Node::LessThanOrEquals(_) => "LessThanOrEquals",
            Node::GreaterThan(_) => "GreaterThan",
            Node::GreaterThanOrEquals(_) => "GreaterThanOrEquals",
            Node::HostFunction(_) => "HostFunction",
            Node::Definition(_) => "Definition",
        }
    }
}

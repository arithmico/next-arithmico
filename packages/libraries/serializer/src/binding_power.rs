use node::{GetNodeType, NodeType};

fn get_binding_power(node_type: NodeType) -> Option<u8> {
    match node_type {
        NodeType::Any => None,
        NodeType::HostFunction
        | NodeType::Tensor
        | NodeType::Number
        | NodeType::Symbol
        | NodeType::Boolean => Some(u8::MAX),
        NodeType::Definition => Some(0),
        NodeType::Function => Some(1),
        NodeType::Or => Some(2),
        NodeType::And => Some(3),
        NodeType::Equals => Some(4),
        NodeType::LessThan => Some(4),
        NodeType::LessThanOrEquals => Some(4),
        NodeType::GreaterThan => Some(4),
        NodeType::GreaterThanOrEquals => Some(4),
        NodeType::Sum => Some(5),
        NodeType::Negate => Some(6),
        NodeType::Product => Some(7),
        // this is correct because division handles parenthesis differently in the divisor
        NodeType::Division => Some(7),
        NodeType::Power => Some(8),
        NodeType::Factorial => Some(9),
        NodeType::FunctionCall => Some(10),
    }
}

pub(crate) trait GetBindingPower {
    fn get_binding_power(&self) -> Option<u8>;
}

impl<T: GetNodeType> GetBindingPower for T {
    fn get_binding_power(&self) -> Option<u8> {
        get_binding_power(self.node_type())
    }
}

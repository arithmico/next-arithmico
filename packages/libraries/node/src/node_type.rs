use crate::Node;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum NodeType {
    Any, // required for user defined functions without type signature
    Boolean,
    Sum,
    Negate,
    Product,
    Division,
    Power,
    Tensor,
    Number,
    Symbol,
    Function,
    FunctionCall,
    And,
    Or,
    Equals,
    LessThan,
    LessThanOrEquals,
    GreaterThan,
    GreaterThanOrEquals,
    HostFunction,
    Definition,
}

impl std::fmt::Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeType::Any => f.write_str("Any"),
            NodeType::Boolean => f.write_str("Boolean"),
            NodeType::Sum => f.write_str("Sum"),
            NodeType::Negate => f.write_str("Negate"),
            NodeType::Product => f.write_str("Product"),
            NodeType::Division => f.write_str("Division"),
            NodeType::Power => f.write_str("Power"),
            NodeType::Tensor => f.write_str("Tensor"),
            NodeType::Number => f.write_str("Number"),
            NodeType::Symbol => f.write_str("Symbol"),
            NodeType::Function => f.write_str("Function"),
            NodeType::FunctionCall => f.write_str("FunctionCall"),
            NodeType::And => f.write_str("And"),
            NodeType::Or => f.write_str("Or"),
            NodeType::Equals => f.write_str("Equals"),
            NodeType::LessThan => f.write_str("LessThan"),
            NodeType::LessThanOrEquals => f.write_str("LessThanOrEquals"),
            NodeType::GreaterThan => f.write_str("GreaterThan"),
            NodeType::GreaterThanOrEquals => f.write_str("GreaterThanOrEquals"),
            NodeType::HostFunction => f.write_str("HostFunction"),
            NodeType::Definition => f.write_str("Definition"),
        }
    }
}

pub trait GetNodeType {
    fn node_type(&self) -> NodeType;
}

impl GetNodeType for Node {
    fn node_type(&self) -> NodeType {
        match self {
            Node::Boolean(_) => NodeType::Boolean,
            Node::Sum(_) => NodeType::Sum,
            Node::Negate(_) => NodeType::Negate,
            Node::Product(_) => NodeType::Product,
            Node::Division(_) => NodeType::Division,
            Node::Power(_) => NodeType::Power,
            Node::Tensor(_) => NodeType::Tensor,
            Node::Number(_) => NodeType::Number,
            Node::Symbol(_) => NodeType::Symbol,
            Node::Function(_) => NodeType::Function,
            Node::FunctionCall(_) => NodeType::FunctionCall,
            Node::And(_) => NodeType::And,
            Node::Or(_) => NodeType::Or,
            Node::Equals(_) => NodeType::Equals,
            Node::LessThan(_) => NodeType::LessThan,
            Node::LessThanOrEquals(_) => NodeType::LessThanOrEquals,
            Node::GreaterThan(_) => NodeType::GreaterThan,
            Node::GreaterThanOrEquals(_) => NodeType::GreaterThanOrEquals,
            Node::HostFunction(_) => NodeType::HostFunction,
            Node::Definition(_) => NodeType::Definition,
        }
    }
}

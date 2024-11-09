use crate::Node;

#[derive(Debug, Clone)]
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

impl ToString for NodeType {
    fn to_string(&self) -> String {
        match self {
            NodeType::Any => String::from("Any"),
            NodeType::Boolean => String::from("Boolean"),
            NodeType::Sum => String::from("Sum"),
            NodeType::Negate => String::from("Negate"),
            NodeType::Product => String::from("Product"),
            NodeType::Division => String::from("Division"),
            NodeType::Power => String::from("Power"),
            NodeType::Tensor => String::from("Tensor"),
            NodeType::Number => String::from("Number"),
            NodeType::Symbol => String::from("Symbol"),
            NodeType::Function => String::from("Function"),
            NodeType::FunctionCall => String::from("FunctionCall"),
            NodeType::And => String::from("And"),
            NodeType::Or => String::from("Or"),
            NodeType::Equals => String::from("Equals"),
            NodeType::LessThan => String::from("LessThan"),
            NodeType::LessThanOrEquals => String::from("LessThanOrEquals"),
            NodeType::GreaterThan => String::from("GreaterThan"),
            NodeType::GreaterThanOrEquals => {
                String::from("GreaterThanOrEquals")
            }
            NodeType::HostFunction => String::from("HostFunction"),
            NodeType::Definition => String::from("Definition"),
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

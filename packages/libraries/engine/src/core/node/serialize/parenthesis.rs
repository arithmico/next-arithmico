use crate::core::node::Node;

pub enum ParenthesesBehavior {
    Optional,
    Required,
}

impl Node {
    pub fn requires_parenthesis(&self, node: &Node) -> ParenthesesBehavior {
        match (self, node) {
            (Node::Number(_) | Node::Symbol(_) | Node::Boolean(_), _) => {
                ParenthesesBehavior::Optional
            }
            (_, Node::Number(_) | Node::Symbol(_) | Node::Boolean(_)) => {
                ParenthesesBehavior::Optional
            }

            (
                Node::Negate(_),
                Node::Product(_) | Node::Division(_) | Node::Power(_),
            ) => ParenthesesBehavior::Optional,

            (
                Node::Sum(_),
                Node::Sum(_)
                | Node::Negate(_)
                | Node::Product(_)
                | Node::Division(_)
                | Node::Power(_),
            ) => ParenthesesBehavior::Optional,

            (
                Node::Product(_),
                Node::Product(_)
                | Node::Division(_)
                | Node::Power(_)
                | Node::Tensor(_)
                | Node::FunctionCall(_),
            ) => ParenthesesBehavior::Optional,

            (
                Node::Division(_),
                Node::Power(_) | Node::Tensor(_) | Node::FunctionCall(_),
            ) => ParenthesesBehavior::Optional,

            (
                Node::Power(_),
                Node::Tensor(_) | Node::FunctionCall(_) | Node::Function(_),
            ) => ParenthesesBehavior::Optional,

            (Node::Tensor(_), _) => ParenthesesBehavior::Optional,

            (Node::FunctionCall(_), _) => ParenthesesBehavior::Optional,

            (Node::Function(_), _) => ParenthesesBehavior::Optional,

            (Node::Definition(_), _) => ParenthesesBehavior::Optional,

            (Node::HostApiFunctionEndpoint(_), _) => {
                ParenthesesBehavior::Optional
            }

            (Node::Or(_), _) => ParenthesesBehavior::Optional,
            (Node::And(_), Node::Or(_)) => ParenthesesBehavior::Required,
            (Node::And(_), _) => ParenthesesBehavior::Optional,

            _ => ParenthesesBehavior::Required,
        }
    }
}

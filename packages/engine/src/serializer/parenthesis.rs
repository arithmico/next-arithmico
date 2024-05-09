use crate::node::Node;

pub enum ParenthesesBehavior {
    Optional,
    Required,
}

impl Node {
    pub fn requires_parenthesis(&self, node: &Node) -> ParenthesesBehavior {
        match (self, node) {
            (
                Node::Number { .. }
                | Node::Symbol { .. }
                | Node::Boolean { .. },
                _,
            ) => ParenthesesBehavior::Optional,
            (
                _,
                Node::Number { .. }
                | Node::Symbol { .. }
                | Node::Boolean { .. },
            ) => ParenthesesBehavior::Optional,

            (
                Node::Negate { .. },
                Node::Product { .. }
                | Node::Division { .. }
                | Node::Power { .. },
            ) => ParenthesesBehavior::Optional,

            (
                Node::Sum { .. },
                Node::Sum { .. }
                | Node::Negate { .. }
                | Node::Product { .. }
                | Node::Division { .. }
                | Node::Power { .. },
            ) => ParenthesesBehavior::Optional,

            (
                Node::Product { .. },
                Node::Product { .. }
                | Node::Division { .. }
                | Node::Power { .. }
                | Node::Vector { .. }
                | Node::FunctionCall { .. },
            ) => ParenthesesBehavior::Optional,

            (
                Node::Division { .. },
                Node::Power { .. }
                | Node::Vector { .. }
                | Node::FunctionCall { .. },
            ) => ParenthesesBehavior::Optional,

            (
                Node::Power { .. },
                Node::Vector { .. }
                | Node::FunctionCall { .. }
                | Node::Function { .. },
            ) => ParenthesesBehavior::Optional,

            (Node::Vector { .. }, _) => ParenthesesBehavior::Optional,

            (Node::FunctionCall { .. }, _) => ParenthesesBehavior::Optional,

            (Node::Function { .. }, _) => ParenthesesBehavior::Optional,

            (Node::Definition { .. }, _) => ParenthesesBehavior::Optional,

            _ => ParenthesesBehavior::Required,
        }
    }
}

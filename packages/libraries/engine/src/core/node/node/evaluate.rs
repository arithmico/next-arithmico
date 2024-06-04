use crate::core::{context::Context, node::*};

impl EvaluateNode for Node {
    fn evaluate(&self, context: &Context) -> Result<Node, NodeError> {
        match self {
            Node::Number(node) if cfg!(feature = "datatype_number") => {
                node.evaluate(context)
            }
            Node::Symbol(node) if cfg!(feature = "datatype_symbol") => {
                node.evaluate(context)
            }
            Node::Boolean(node) if cfg!(feature = "datatype_boolean") => {
                node.evaluate(context)
            }
            Node::Negate(node) => node.evaluate(context),
            Node::Sum(node) => node.evaluate(context),
            Node::Product(node) => node.evaluate(context),
            Node::Division(node) => node.evaluate(context),
            Node::Power(node) => node.evaluate(context),
            Node::Function(node) if cfg!(feature = "datatype_function") => {
                node.evaluate(context)
            }
            Node::FunctionCall(node)
                if cfg!(feature = "operator_function_call") =>
            {
                node.evaluate(context)
            }
            Node::Definition(node) if cfg!(feature = "operator_definition") => {
                node.evaluate(context)
            }
            Node::Tensor(node) if cfg!(feature = "datatype_vector") => {
                node.evaluate(context)
            }
            Node::And(node) => node.evaluate(context),
            Node::Or(node) => node.evaluate(context),
            Node::Equals(node) => node.evaluate(context),
            Node::LessThan(node) => node.evaluate(context),
            Node::GreaterThan(node) => node.evaluate(context),
            _ => Err(NodeError::UnsupportedOperation),
        }
    }
}

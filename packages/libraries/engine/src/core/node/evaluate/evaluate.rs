use crate::core::{context::Context, node::Node};

use super::nodes::{
    evaluate_definition, evaluate_division, evaluate_function,
    evaluate_function_call, evaluate_negate, evaluate_number, evaluate_power,
    evaluate_product, evaluate_sum, evaluate_symbol, evaluate_tensor,
};
use super::NodeEvaluationError;

impl Node {
    pub fn evaluate(
        &self,
        context: &Context,
    ) -> Result<Node, NodeEvaluationError> {
        match self {
            Node::Number(node) if cfg!(feature = "datatype_number") => {
                evaluate_number(node, context)
            }
            Node::Symbol(node) if cfg!(feature = "datatype_symbol") => {
                evaluate_symbol(node, context)
            }
            Node::Negate(node) => evaluate_negate(node, context),
            Node::Sum(node) => evaluate_sum(node, context),
            Node::Product(node) => evaluate_product(node, context),
            Node::Division(node) => evaluate_division(node, context),
            Node::Power(node) => evaluate_power(node, context),
            Node::Function(node) if cfg!(feature = "datatype_function") => {
                evaluate_function(node, context)
            }
            Node::FunctionCall(node)
                if cfg!(feature = "operator_function_call") =>
            {
                evaluate_function_call(node, context)
            }
            Node::Definition(node) if cfg!(feature = "operator_definition") => {
                evaluate_definition(node, context)
            }
            Node::Tensor(node) if cfg!(feature = "datatype_vector") => {
                evaluate_tensor(node, context)
            }
            _ => Err(NodeEvaluationError::UnsupportedOperation),
        }
    }
}

use crate::core::node::{
    Definition, Division, Function, FunctionCall, Negate, Number, Power,
    Product, Sum, Symbol, Tensor,
};
use crate::core::{context::Context, node::Node};

use super::nodes::{
    evaluate_definition, evaluate_division, evaluate_function,
    evaluate_function_call, evaluate_negate, evaluate_number, evaluate_power,
    evaluate_product, evaluate_sum, evaluate_symbol, evaluate_vector,
};
use super::NodeEvaluationError;

impl Node {
    pub fn evaluate(
        &self,
        context: &Context,
    ) -> Result<Node, NodeEvaluationError> {
        match self {
            Node::Number(Number { value })
                if cfg!(feature = "datatype_number") =>
            {
                evaluate_number(value, context)
            }
            Node::Symbol(Symbol { name })
                if cfg!(feature = "datatype_symbol") =>
            {
                evaluate_symbol(name, context)
            }
            Node::Negate(Negate { value }) => evaluate_negate(value, context),
            Node::Sum(Sum { values }) => evaluate_sum(values, context),
            Node::Product(Product { values }) => {
                evaluate_product(values, context)
            }
            Node::Division(Division { dividend, divisor }) => {
                evaluate_division(dividend, divisor, context)
            }
            Node::Power(Power { base, exponent }) => {
                evaluate_power(base, exponent, context)
            }
            Node::Function(Function {
                arguments,
                expression,
            }) if cfg!(feature = "datatype_function") => {
                evaluate_function(arguments, expression, context)
            }
            Node::FunctionCall(FunctionCall { target, arguments })
                if cfg!(feature = "operator_function_call") =>
            {
                evaluate_function_call(target, arguments, context)
            }
            Node::Definition(Definition { symbol, expression })
                if cfg!(feature = "operator_definition") =>
            {
                evaluate_definition(symbol, expression, context)
            }
            Node::Tensor(Tensor { elements: values })
                if cfg!(feature = "datatype_vector") =>
            {
                evaluate_vector(values, context)
            }
            _ => Err(NodeEvaluationError::UnsupportedOperation),
        }
    }
}

use ast::Node;

use crate::{
    context::Context, error::EvaluateNodeError, evaluate::EvaluateNode,
};

mod boolean;
mod number;
mod sum;
mod symbol;

impl EvaluateNode for Node {
    fn evaluate(&self, context: &Context) -> Result<Node, EvaluateNodeError> {
        match self {
            Node::Boolean(boolean) => boolean.evaluate(context),
            Node::Sum(sum) => sum.evaluate(context),
            Node::Negate(negate) => todo!(),
            Node::Product(product) => todo!(),
            Node::Division(division) => todo!(),
            Node::Power(power) => todo!(),
            Node::Tensor(tensor) => todo!(),
            Node::Number(number) => number.evaluate(context),
            Node::Symbol(symbol) => symbol.evaluate(context),
            Node::Function(function) => todo!(),
            Node::FunctionCall(function_call) => todo!(),
            Node::And(and) => todo!(),
            Node::Or(or) => todo!(),
            Node::Equals(equals) => todo!(),
            Node::LessThan(less_than) => todo!(),
            Node::LessThanOrEquals(less_than_or_equals) => todo!(),
            Node::GreaterThan(greater_than) => todo!(),
            Node::GreaterThanOrEquals(greater_than_or_equals) => todo!(),
            Node::HostFunction(host_function) => todo!(),
            Node::Definition(definition) => todo!(),
        }
    }
}

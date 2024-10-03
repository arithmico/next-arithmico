use ast::Node;

use crate::{
    context::Context, error::EvaluateNodeError, evaluate::EvaluateNode,
};

mod and;
mod boolean;
mod definition;
mod division;
mod equals;
mod function;
mod negate;
mod number;
mod or;
mod power;
mod product;
mod sum;
mod symbol;
mod tensor;

impl EvaluateNode for Node {
    fn evaluate(&self, context: &Context) -> Result<Node, EvaluateNodeError> {
        match self {
            Node::Boolean(boolean) => boolean.evaluate(context),
            Node::Sum(sum) => sum.evaluate(context),
            Node::Negate(negate) => negate.evaluate(context),
            Node::Product(product) => product.evaluate(context),
            Node::Division(division) => division.evaluate(context),
            Node::Power(power) => power.evaluate(context),
            Node::Tensor(tensor) => tensor.evaluate(context),
            Node::Number(number) => number.evaluate(context),
            Node::Symbol(symbol) => symbol.evaluate(context),
            Node::Function(function) => function.evaluate(context),
            Node::FunctionCall(function_call) => todo!(),
            Node::And(and) => and.evaluate(context),
            Node::Or(or) => or.evaluate(context),
            Node::Equals(equals) => equals.evaluate(context),
            Node::LessThan(less_than) => todo!(),
            Node::LessThanOrEquals(less_than_or_equals) => todo!(),
            Node::GreaterThan(greater_than) => todo!(),
            Node::GreaterThanOrEquals(greater_than_or_equals) => todo!(),
            Node::HostFunction(host_function) => todo!(),
            Node::Definition(definition) => definition.evaluate(context),
        }
    }
}

use crate::{EvaluateNodeContext, EvaluateNodeError, Node};
use trace::TracableMut;

use crate::evaluate::EvaluateNode;

mod and;
mod boolean;
mod definition;
mod division;
mod equals;
mod function;
mod function_call;
mod greater_than;
mod greater_than_or_equals;
mod host_function;
mod less_than;
mod less_than_or_equals;
mod negate;
mod number;
mod or;
mod power;
mod product;
mod sum;
mod symbol;
mod tensor;

impl EvaluateNode for Node {
    fn evaluate(
        &self,
        context: &EvaluateNodeContext,
    ) -> Result<Node, EvaluateNodeError> {
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
            Node::FunctionCall(function_call) => {
                function_call.evaluate(context)
            }
            Node::And(and) => and.evaluate(context),
            Node::Or(or) => or.evaluate(context),
            Node::Equals(equals) => equals.evaluate(context),
            Node::LessThan(less_than) => less_than.evaluate(context),
            Node::LessThanOrEquals(less_than_or_equals) => {
                less_than_or_equals.evaluate(context)
            }
            Node::GreaterThan(greater_than) => greater_than.evaluate(context),
            Node::GreaterThanOrEquals(greater_than_or_equals) => {
                greater_than_or_equals.evaluate(context)
            }
            Node::HostFunction(host_function) => {
                host_function.evaluate(context)
            }
            Node::Definition(definition) => definition.evaluate(context),
        }
        .map(|node| node.with_tracable(self))
        .map_err(|error| error.with_tracable(self))
    }
}

use node::Node;
use trace::{Tracable, TracableMut};

use crate::core::{Context, EvaluateNode, EvaluateNodeError};

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
            Node::Factorial(factorial) => factorial.evaluate(context),
        }
        .map(|node| node.with_optional_span(self.hull()))
        .map_err(|error| error.with_optional_new_frame(self.hull()))
    }
}

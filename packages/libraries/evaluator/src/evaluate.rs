use node::Node;
use trace::{Tracable, TracableMut};

use crate::{Error, Options};

mod and;
mod boolean;
mod definition;
mod division;
mod equals;
mod factorial;
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

pub trait EvaluateNode {
    fn evaluate<'a>(&'a self, options: Options<'a>) -> Result<Node, Error>;
}

impl EvaluateNode for Node {
    fn evaluate<'a>(&'a self, options: Options<'a>) -> Result<Node, Error> {
        match self {
            Node::Boolean(boolean) => boolean.evaluate(options),
            Node::Sum(sum) => sum.evaluate(options),
            Node::Negate(negate) => negate.evaluate(options),
            Node::Product(product) => product.evaluate(options),
            Node::Division(division) => division.evaluate(options),
            Node::Power(power) => power.evaluate(options),
            Node::Tensor(tensor) => tensor.evaluate(options),
            Node::Number(number) => number.evaluate(options),
            Node::Symbol(symbol) => symbol.evaluate(options),
            Node::Function(function) => function.evaluate(options),
            Node::FunctionCall(function_call) => {
                function_call.evaluate(options)
            }
            Node::And(and) => and.evaluate(options),
            Node::Or(or) => or.evaluate(options),
            Node::Equals(equals) => equals.evaluate(options),
            Node::LessThan(less_than) => less_than.evaluate(options),
            Node::LessThanOrEquals(less_than_or_equals) => {
                less_than_or_equals.evaluate(options)
            }
            Node::GreaterThan(greater_than) => greater_than.evaluate(options),
            Node::GreaterThanOrEquals(greater_than_or_equals) => {
                greater_than_or_equals.evaluate(options)
            }
            Node::HostFunction(host_function) => {
                host_function.evaluate(options)
            }
            Node::Definition(definition) => definition.evaluate(options),
            Node::Factorial(factorial) => factorial.evaluate(options),
        }
        .map(|node| node.with_optional_span(self.hull()))
        .map_err(|error| error.with_optional_new_frame(self.hull()))
    }
}

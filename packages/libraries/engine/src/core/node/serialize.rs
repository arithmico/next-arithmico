use crate::core::{
    Context, Node, NormalizeNode, Serialize, SerializeNodeError,
};

impl NormalizeNode for Node {
    fn normalize_node(
        &self,
        context: &Context,
    ) -> Result<Node, SerializeNodeError> {
        match self {
            Node::Boolean(boolean) => boolean.normalize_node(context),
            Node::Sum(sum) => sum.normalize_node(context),
            Node::Negate(negate) => negate.normalize_node(context),
            Node::Product(product) => product.normalize_node(context),
            Node::Division(division) => division.normalize_node(context),
            Node::Power(power) => power.normalize_node(context),
            Node::Tensor(tensor) => tensor.normalize_node(context),
            Node::Number(number) => number.normalize_node(context),
            Node::Symbol(symbol) => symbol.normalize_node(context),
            Node::Function(function) => function.normalize_node(context),
            Node::FunctionCall(function_call) => {
                function_call.normalize_node(context)
            }
            Node::And(and) => and.normalize_node(context),
            Node::Or(or) => or.normalize_node(context),
            Node::Equals(equals) => equals.normalize_node(context),
            Node::LessThan(less_than) => less_than.normalize_node(context),
            Node::LessThanOrEquals(less_than_or_equals) => {
                less_than_or_equals.normalize_node(context)
            }
            Node::GreaterThan(greater_than) => {
                greater_than.normalize_node(context)
            }
            Node::GreaterThanOrEquals(greater_than_or_equals) => {
                greater_than_or_equals.normalize_node(context)
            }
            Node::HostFunction(host_function) => {
                host_function.normalize_node(context)
            }
            Node::Definition(definition) => definition.normalize_node(context),
        }
    }
}

impl Serialize for Node {
    fn serialize(
        &self,
        context: &Context,
    ) -> Result<String, SerializeNodeError> {
        match self {
            Node::Boolean(boolean) => boolean.serialize(context),
            Node::Sum(sum) => sum.serialize(context),
            Node::Negate(negate) => negate.serialize(context),
            Node::Product(product) => product.serialize(context),
            Node::Division(division) => division.serialize(context),
            Node::Power(power) => power.serialize(context),
            Node::Tensor(tensor) => tensor.serialize(context),
            Node::Number(number) => number.serialize(context),
            Node::Symbol(symbol) => symbol.serialize(context),
            Node::Function(function) => function.serialize(context),
            Node::FunctionCall(function_call) => {
                function_call.serialize(context)
            }
            Node::And(and) => and.serialize(context),
            Node::Or(or) => or.serialize(context),
            Node::Equals(equals) => equals.serialize(context),
            Node::LessThan(less_than) => less_than.serialize(context),
            Node::LessThanOrEquals(less_than_or_equals) => {
                less_than_or_equals.serialize(context)
            }
            Node::GreaterThan(greater_than) => greater_than.serialize(context),
            Node::GreaterThanOrEquals(greater_than_or_equals) => {
                greater_than_or_equals.serialize(context)
            }
            Node::HostFunction(host_function) => {
                host_function.serialize(context)
            }
            Node::Definition(definition) => definition.serialize(context),
        }
    }
}

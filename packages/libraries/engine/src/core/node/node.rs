use crate::core::context::Context;
use std::fmt::Debug;

use super::{
    evaluate::NodeEvaluationError, nodes::*,
    serialize::parenthesis::ParenthesesBehavior,
};

#[derive(PartialEq, Debug, Clone)]
pub enum Node {
    Number(Number),
    Symbol(Symbol),
    Boolean(Boolean),
    Negate(Negate),
    Sum(Sum),
    Product(Product),
    Division(Division),
    Power(Power),
    Tensor(Tensor),
    FunctionCall(FunctionCall),
    Function(Function),
    HostApiFunctionEndpoint(HostFunction),
    Definition(Definition),
    And(And),
    Or(Or),
}

pub trait BaseNode: Into<Node> + PartialEq + Debug + Clone {}
pub trait ComputableNode: BaseNode {
    fn compute(&self, context: &Context) -> Result<Node, NodeEvaluationError>;
}

pub trait SerializeableNode: BaseNode {
    fn child_requires_parenthesis(&self, child: &Node) -> ParenthesesBehavior;
    fn pre_serialize_transform(&self, context: &Context) -> Node;
}

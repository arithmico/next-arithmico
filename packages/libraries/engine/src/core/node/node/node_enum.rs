use std::fmt::Debug;

use crate::core::node::{nodes::*, BaseNode};

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
    Equals(Equals),
}

impl BaseNode for Node {}

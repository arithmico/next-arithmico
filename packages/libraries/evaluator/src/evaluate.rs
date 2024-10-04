use ast::Node;

use crate::{context::EvaluateNodeContext, error::EvaluateNodeError};

pub trait EvaluateNode {
    fn evaluate(
        &self,
        context: &EvaluateNodeContext,
    ) -> Result<Node, EvaluateNodeError>;
}

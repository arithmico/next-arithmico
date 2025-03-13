use crate::{EvaluateNodeContext, EvaluateNodeError, Node};

pub trait EvaluateNode {
    fn evaluate(
        &self,
        context: &EvaluateNodeContext,
    ) -> Result<Node, EvaluateNodeError>;
}

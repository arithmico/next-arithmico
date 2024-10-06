use ast::Node;
use common::{EvaluateNodeContext, EvaluateNodeError};

pub trait EvaluateNode {
    fn evaluate(
        &self,
        context: &EvaluateNodeContext,
    ) -> Result<Node, EvaluateNodeError>;
}

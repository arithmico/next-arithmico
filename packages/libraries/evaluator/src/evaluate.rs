use ast::Node;

use crate::error::EvaluationError;

pub trait EvaluateNode {
    fn evaluate(&self, context: &Context) -> Result<Node, EvaluationError>;
}

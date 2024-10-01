use ast::Node;

use crate::{context::Context, error::EvaluationError};

pub trait EvaluateNode {
    fn evaluate(&self, context: &Context) -> Result<Node, EvaluationError>;
}

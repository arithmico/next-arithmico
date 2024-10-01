use ast::Node;

use crate::{context::Context, error::EvaluateNodeError};

pub trait EvaluateNode {
    fn evaluate(&self, context: &Context) -> Result<Node, EvaluateNodeError>;
}

use crate::{EvaluateNodeContext, EvaluateNodeError, Node};

mod map_function_arguments;

pub use map_function_arguments::*;

pub trait EvaluateNode {
    fn evaluate(
        &self,
        context: &EvaluateNodeContext,
    ) -> Result<Node, EvaluateNodeError>;
}

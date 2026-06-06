use crate::core::{Context, EvaluateNodeError, Node};

mod map_function_arguments;

pub use map_function_arguments::*;

pub trait EvaluateNode {
    fn evaluate(&self, context: &Context) -> Result<Node, EvaluateNodeError>;
}

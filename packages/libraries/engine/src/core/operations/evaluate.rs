use node::Node;

use crate::core::Context;

mod map_function_arguments;

pub use map_function_arguments::*;

pub trait EvaluateNode {
    fn evaluate(&self, context: &Context) -> Result<Node, evaluator::Error>;
}

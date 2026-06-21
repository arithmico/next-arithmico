use node::{HostFunction, Node};

use crate::core::{Context, EvaluateNode, EvaluateNodeError};

impl EvaluateNode for node::HostFunction {
    fn evaluate(&self, _context: &Context) -> Result<Node, EvaluateNodeError> {
        Ok(HostFunction::new(&self.name))
    }
}

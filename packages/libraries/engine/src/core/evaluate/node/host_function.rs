use crate::{EvaluateNodeContext, EvaluateNodeError, HostFunction, Node};

use crate::core::evaluate::EvaluateNode;

impl EvaluateNode for HostFunction {
    fn evaluate(
        &self,
        _context: &EvaluateNodeContext,
    ) -> Result<Node, EvaluateNodeError> {
        Ok(HostFunction::new(&self.name))
    }
}

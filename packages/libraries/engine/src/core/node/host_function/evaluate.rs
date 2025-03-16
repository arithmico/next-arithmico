use crate::core::{
    EvaluateNode, EvaluateNodeContext, EvaluateNodeError, HostFunction, Node,
};

impl EvaluateNode for HostFunction {
    fn evaluate(
        &self,
        _context: &EvaluateNodeContext,
    ) -> Result<Node, EvaluateNodeError> {
        Ok(HostFunction::new(&self.name))
    }
}

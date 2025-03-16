use crate::core::{
    EvaluateNode, Context, EvaluateNodeError, HostFunction, Node,
};

impl EvaluateNode for HostFunction {
    fn evaluate(
        &self,
        _context: &Context,
    ) -> Result<Node, EvaluateNodeError> {
        Ok(HostFunction::new(&self.name))
    }
}

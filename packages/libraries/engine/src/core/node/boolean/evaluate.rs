use node::{Boolean, GetNodeType, Node};

use crate::core::{Context, EvaluateNode, EvaluateNodeError};

impl EvaluateNode for Boolean {
    fn evaluate(&self, _context: &Context) -> Result<Node, EvaluateNodeError> {
        if !cfg!(feature = "datatype_boolean") {
            return Err(EvaluateNodeError::unsupported_datatype(
                self.node_type(),
            ));
        }

        Ok(Boolean::new(self.value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use trace::TracableMut;
    use trace::Trace;

    #[test]
    fn evaluate_boolean_true() {
        let context = Context::default();
        let result = Boolean::new(true).evaluate(&context).unwrap();
        assert_eq!(result, Boolean::new(true));
    }

    #[test]
    fn evaluate_boolean_false() {
        let context = Context::default();
        let result = Boolean::new(false).evaluate(&context).unwrap();
        assert_eq!(result, Boolean::new(false));
    }

    #[test]
    fn evaluate_boolean_with_trace() {
        let context = Context::default();
        let trace = Trace::new().with_span(0, 4);
        let result = Boolean::new(false)
            .with_trace(&trace)
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Boolean::new(false).with_trace(&trace));
    }
}

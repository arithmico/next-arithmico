use crate::evaluate::EvaluateNode;
use ast::{Boolean, Node};
use common::{EvaluateNodeContext, EvaluateNodeError};

impl EvaluateNode for Boolean {
    fn evaluate(
        &self,
        _context: &EvaluateNodeContext,
    ) -> Result<Node, EvaluateNodeError> {
        if !cfg!(feature = "datatype_boolean") {
            return Err(EvaluateNodeError::unsupported_datatype("Boolean"));
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
        let context = EvaluateNodeContext::default();
        let result = Boolean::new(true).evaluate(&context).unwrap();
        assert_eq!(result, Boolean::new(true));
    }

    #[test]
    fn evaluate_boolean_false() {
        let context = EvaluateNodeContext::default();
        let result = Boolean::new(false).evaluate(&context).unwrap();
        assert_eq!(result, Boolean::new(false));
    }

    #[test]
    fn evaluate_boolean_with_trace() {
        let context = EvaluateNodeContext::default();
        let trace = Trace::new().with_span(0, 4);
        let result = Boolean::new(false)
            .with_trace(&trace)
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Boolean::new(false).with_trace(&trace));
    }
}

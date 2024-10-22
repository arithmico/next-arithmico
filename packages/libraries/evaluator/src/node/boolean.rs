use crate::evaluate::EvaluateNode;
use ast::{Boolean, Node};
use common::{EvaluateNodeContext, EvaluateNodeError};
use trace::{Tracable, TracableMut};

impl EvaluateNode for Boolean {
    fn evaluate(
        &self,
        _context: &EvaluateNodeContext,
    ) -> Result<Node, EvaluateNodeError> {
        if !cfg!(feature = "datatype_boolean") {
            return Err(EvaluateNodeError::UnsupportedDataType(String::from(
                "boolean",
            )));
        }

        Ok(Boolean::new(self.value).with_trace(self.trace().clone()))
    }
}

#[cfg(test)]
mod tests {
    use trace::Trace;

    use super::*;

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
            .with_trace(trace.clone())
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Boolean::new(false).with_trace(trace));
    }
}

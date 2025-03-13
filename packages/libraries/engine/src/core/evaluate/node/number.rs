use crate::core::evaluate::EvaluateNode;
use crate::{
    EvaluateNodeContext, EvaluateNodeError, GetNodeType, Node, Number,
};

impl EvaluateNode for Number {
    fn evaluate(
        &self,
        _context: &EvaluateNodeContext,
    ) -> Result<Node, EvaluateNodeError> {
        if !cfg!(feature = "datatype_number") {
            return Err(EvaluateNodeError::unsupported_datatype(
                self.node_type(),
            ));
        }

        Ok(Number::new(self.value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use trace::TracableMut;

    #[test]
    fn evaluate_number() {
        let context = EvaluateNodeContext::default();
        let result = Number::new(1.2345).evaluate(&context).unwrap();
        assert_eq!(result, Number::new(1.2345));
    }

    #[test]
    fn evaluate_number_with_trace() {
        let context = EvaluateNodeContext::default();
        let result = Number::new(1.2345)
            .with_span(0, 5)
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Number::new(1.2345).with_span(0, 5));
    }
}

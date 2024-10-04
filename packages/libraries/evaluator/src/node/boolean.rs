use ast::{Boolean, Node};

use crate::{
    context::EvaluateNodeContext, error::EvaluateNodeError,
    evaluate::EvaluateNode,
};

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

        Ok(Boolean::new(self.value))
    }
}

#[cfg(test)]
mod tests {
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
}

use ast::{Node, Number};

use crate::{evaluate::EvaluateNode, EvaluateNodeContext, EvaluateNodeError};

impl EvaluateNode for Number {
    fn evaluate(
        &self,
        _context: &EvaluateNodeContext,
    ) -> Result<Node, EvaluateNodeError> {
        if !cfg!(feature = "datatype_number") {
            return Err(EvaluateNodeError::UnsupportedDataType(String::from(
                "number",
            )));
        }

        Ok(Number::new(self.value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_number() {
        let context = EvaluateNodeContext::default();
        let result = Number::new(1.2345).evaluate(&context).unwrap();
        assert_eq!(result, Number::new(1.2345));
    }
}

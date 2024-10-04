use ast::{Definition, Node};

use crate::{evaluate::EvaluateNode, EvaluateNodeContext, EvaluateNodeError};

impl EvaluateNode for Definition {
    fn evaluate(
        &self,
        context: &EvaluateNodeContext,
    ) -> Result<Node, EvaluateNodeError> {
        if !cfg!(feature = "operator_definition") {
            return Err(EvaluateNodeError::UnsupportedOperation);
        }

        let expression = self.expression.evaluate(context)?;
        Ok(Definition::new(&self.symbol, expression))
    }
}

#[cfg(test)]
mod tests {
    use ast::{Number, Sum};

    use super::*;

    #[test]
    fn evaluate_definition() {
        let context = EvaluateNodeContext::default();
        let result = Definition::new(
            "test",
            Sum::new(vec![Number::new(1.), Number::new(2.)]),
        )
        .evaluate(&context)
        .unwrap();
        assert_eq!(result, Definition::new("test", Number::new(3.)));
    }
}

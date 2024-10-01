use ast::{Node, Number};

use crate::{evaluate::EvaluateNode, Context, EvaluateNodeError};

impl EvaluateNode for Number {
    fn evaluate(&self, _context: &Context) -> Result<Node, EvaluateNodeError> {
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
        let context = Context::default();
        let result = Number::new(1.2345).evaluate(&context).unwrap();
        assert_eq!(result, Number::new(1.2345));
    }
}

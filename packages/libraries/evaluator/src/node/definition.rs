use ast::{Definition, Node};

use crate::{evaluate::EvaluateNode, Context, EvaluateNodeError};

impl EvaluateNode for Definition {
    fn evaluate(&self, context: &Context) -> Result<Node, EvaluateNodeError> {
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
        let context = Context::default();
        let result = Definition::new(
            "test",
            Sum::new(vec![Number::new(1.), Number::new(2.)]),
        )
        .evaluate(&context)
        .unwrap();
        assert_eq!(result, Definition::new("test", Number::new(3.)));
    }
}

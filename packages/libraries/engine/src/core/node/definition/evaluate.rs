use evaluator::Error;
use node::{Definition, Node};

use crate::core::{Context, EvaluateNode};

impl EvaluateNode for Definition {
    fn evaluate(&self, context: &Context) -> Result<Node, Error> {
        if !cfg!(feature = "operator_definition") {
            return Err(Error::unsupported_operation());
        }

        let expression = self.expression.evaluate(context)?;
        Ok(Definition::new(&self.symbol, expression))
    }
}

#[cfg(test)]
mod tests {
    use node::{Number, Sum};

    use super::*;

    #[test]
    fn evaluate_definition() {
        let context = Context::default();
        let result = Definition::new(
            "test",
            Sum::new(vec![Number::new_node(1.), Number::new_node(2.)]),
        )
        .evaluate(&context)
        .unwrap();
        assert_eq!(result, Definition::new("test", Number::new_node(3.)));
    }
}

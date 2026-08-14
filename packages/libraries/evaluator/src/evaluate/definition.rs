use node::{Definition, Node};

use crate::{Error, EvaluateNode, Options};

impl EvaluateNode for Definition {
    fn evaluate(&self, context: Options) -> Result<Node, Error> {
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

    use crate::{Api, Stack};

    use super::*;

    #[test]
    fn evaluate_definition() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = Definition::new(
            "test",
            Sum::new(vec![Number::new_node(1.), Number::new_node(2.)]),
        )
        .evaluate(options)
        .unwrap();
        assert_eq!(result, Definition::new("test", Number::new_node(3.)));
    }
}

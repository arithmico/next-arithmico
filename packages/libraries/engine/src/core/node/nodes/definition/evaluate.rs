use crate::core::{context::Context, node::*};

impl EvaluateNode for Definition {
    fn evaluate(&self, context: &Context) -> Result<Node, NodeError> {
        let evaluated_expression = self.expression.evaluate(context)?;
        Ok(Definition::new(self.symbol.clone(), evaluated_expression).into())
    }
}

#[cfg(test)]
mod tests {
    use crate::core::node::nodes::Number;

    use super::*;

    #[test]
    fn evaluate_definition() {
        let context = Context::default();
        assert_eq!(
            Node::from(Definition::new("test", Number::new(42.0)))
                .evaluate(&context)
                .unwrap(),
            Definition::new("test", Number::new(42.0)).into()
        )
    }
}

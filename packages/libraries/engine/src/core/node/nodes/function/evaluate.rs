use crate::core::{context::Context, node::*};

impl EvaluateNode for Function {
    fn evaluate(
        &self,
        _context: &Context,
    ) -> Result<Node, EvaluateNodeError> {
        Ok(
            Function::new(self.arguments.clone(), (*self.expression).clone())
                .into(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_function() {
        let context = Context::default();
        assert_eq!(
            Node::from(Function::new(vec!["x".into()], Symbol::new("x")))
                .evaluate(&context)
                .unwrap(),
            Node::from(Function::new(vec!["x".into()], Symbol::new("x")))
                .into()
        )
    }
}

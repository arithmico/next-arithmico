use crate::core::{context::Context, node::*};

impl EvaluateNode for Or {
    fn evaluate(&self, context: &Context) -> Result<Node, EvaluateNodeError> {
        self.values
            .iter()
            .map(|value| value.evaluate(context))
            .reduce(|acc, val| {
                let left = acc?;
                let right = val?;
                match (left, right) {
                    (Node::Boolean(left), Node::Boolean(right))
                        if cfg!(feature = "operator_or_boolean_boolean") =>
                    {
                        Ok(Boolean::new(left.value || right.value).into())
                    }
                    _ => Err(EvaluateNodeError::UnsupportedOperation),
                }
            })
            .unwrap_or(Err(EvaluateNodeError::InvalidNumberOfValues))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn evaluate_or_boolean_boolean_1() {
        let context = Context::default();
        assert_eq!(
            Node::from(Or::new(vec![
                Boolean::new(true).into(),
                Boolean::new(true).into(),
            ]))
            .evaluate(&context)
            .unwrap(),
            Boolean::new(true).into(),
        )
    }

    #[test]
    fn evaluate_or_boolean_boolean_2() {
        let context = Context::default();
        assert_eq!(
            Node::from(Or::new(vec![
                Boolean::new(false).into(),
                Boolean::new(true).into(),
            ]))
            .evaluate(&context)
            .unwrap(),
            Boolean::new(true).into(),
        )
    }

    #[test]
    fn evaluate_or_boolean_boolean_3() {
        let context = Context::default();
        assert_eq!(
            Node::from(Or::new(vec![
                Boolean::new(true).into(),
                Boolean::new(false).into(),
            ]))
            .evaluate(&context)
            .unwrap(),
            Boolean::new(true).into(),
        )
    }

    #[test]
    fn evaluate_or_boolean_boolean_4() {
        let context = Context::default();
        assert_eq!(
            Node::from(Or::new(vec![
                Boolean::new(false).into(),
                Boolean::new(false).into(),
            ]))
            .evaluate(&context)
            .unwrap(),
            Boolean::new(false).into(),
        )
    }

    #[test]
    fn evaluate_or_boolean_boolean_5() {
        let context = Context::default();
        assert_eq!(
            Node::from(Or::new(vec![
                Boolean::new(false).into(),
                Boolean::new(false).into(),
                Boolean::new(false).into(),
                Boolean::new(false).into(),
                Boolean::new(false).into(),
                Boolean::new(true).into(),
            ]))
            .evaluate(&context)
            .unwrap(),
            Boolean::new(true).into(),
        )
    }
}

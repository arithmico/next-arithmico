use crate::core::{context::Context, node::*};

use super::Boolean;

impl EvaluateNode for Boolean {
    fn evaluate(&self, _context: &Context) -> Result<Node, NodeError> {
        Ok(Boolean::new(self.value).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_boolean_true() {
        let context = Context::default();
        assert_eq!(
            Node::from(Boolean::new(true)).evaluate(&context).unwrap(),
            Node::from(Boolean::new(true))
        )
    }

    #[test]
    fn evaluate_boolean_false() {
        let context = Context::default();
        assert_eq!(
            Node::from(Boolean::new(false)).evaluate(&context).unwrap(),
            Node::from(Boolean::new(false))
        )
    }
}

use crate::core::{context::Context, node::*};

use super::Number;

impl EvaluateNode for Number {
    fn evaluate(&self, _context: &Context) -> Result<Node, NodeError> {
        Ok(Number::new(self.value).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_number() {
        let context = Context::default();
        assert_eq!(
            Node::from(Number::new(2.1)).evaluate(&context).unwrap(),
            Node::from(Number::new(2.1))
        )
    }
}

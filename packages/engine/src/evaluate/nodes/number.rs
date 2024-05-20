use crate::{context::Context, evaluate::NodeEvaluationError, node::Node};

pub fn evaluate_number(
    value: &f64,
    _context: &Context,
) -> Result<Node, NodeEvaluationError> {
    Ok(Node::Number {
        value: f64::clone(value),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_number() {
        let context = Context::default();
        assert_eq!(
            Node::Number { value: 2.1 }.evaluate(&context).unwrap(),
            Node::Number { value: 2.1 }
        )
    }
}

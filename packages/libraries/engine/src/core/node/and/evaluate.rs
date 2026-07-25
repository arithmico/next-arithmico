use node::{And, Boolean, GetNodeType, Node};
use trace::{CombineHulls, Tracable, TracableMut};

use crate::core::{Context, EvaluateNode, EvaluateNodeError};

impl EvaluateNode for And {
    fn evaluate(&self, context: &Context) -> Result<Node, EvaluateNodeError> {
        if self.elements.len() < 2 {
            return Err(EvaluateNodeError::invalid_node(self.node_type()));
        }

        self.elements
            .iter()
            .map(|element| element.evaluate(context))
            .reduce(|left, right| combine_and_elements(&left?, &right?))
            // Safety: this can not panic due to the previous length check
            .expect("min 2 elements")
    }
}

fn combine_and_elements(
    left: &Node,
    right: &Node,
) -> Result<Node, EvaluateNodeError> {
    match (left, right) {
        (Node::Boolean(left), Node::Boolean(right))
            if cfg!(feature = "operator_and_boolean_boolean") =>
        {
            let span = (left, right).combine_hulls();
            Ok(
                Boolean::new(left.value && right.value)
                    .with_optional_span(span),
            )
        }
        (left, right) => Err(EvaluateNodeError::unsupported_operation()
            .with_optional_span(left.hull())
            .with_optional_span(right.hull())),
    }
}

#[cfg(test)]
mod tests {
    use lexer::Span;
    use node::NodeType;

    use super::*;

    #[test]
    fn evaluate_invalid_and() {
        let context = Context::default();
        let result = And::new(vec![Boolean::new(true)]).evaluate(&context);
        assert_eq!(result, Err(EvaluateNodeError::invalid_node(NodeType::And)));
    }

    #[test]
    fn evaluate_and_boolean_boolean_2() {
        let context = Context::default();
        let result = And::new(vec![Boolean::new(false), Boolean::new(true)])
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Boolean::new(false));
    }

    #[test]
    fn evaluate_and_boolean_boolean_3() {
        let context = Context::default();
        let result = And::new(vec![
            Boolean::new(true),
            Boolean::new(true),
            Boolean::new(true),
        ])
        .evaluate(&context)
        .unwrap();
        assert_eq!(result, Boolean::new(true));
    }

    #[test]
    fn evaluate_and_with_trace() {
        let context = Context::default();
        let result = And::new(vec![
            Boolean::new(false).with_span(Span::new_between(0, 0)),
            Boolean::new(true).with_span(Span::new_between(2, 2)),
        ])
        .with_span(Span::new_between(0, 2))
        .evaluate(&context)
        .unwrap();
        assert_eq!(
            result,
            Boolean::new(false).with_span(Span::new_between(0, 2))
        );
    }
}

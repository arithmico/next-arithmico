use node::{Boolean, GetNodeType, Node, Or};
use trace::TracableMut;

use crate::core::{Context, EvaluateNode, EvaluateNodeError};

impl EvaluateNode for Or {
    fn evaluate(&self, context: &Context) -> Result<Node, EvaluateNodeError> {
        if self.elements.len() < 2 {
            return Err(EvaluateNodeError::invalid_node(self.node_type()));
        }

        let mut elements = self
            .elements
            .iter()
            .map(|element| element.evaluate(context));

        let mut accumulator = elements.next().unwrap()?;
        for current_element in elements {
            accumulator = combine_or_elements(&accumulator, &current_element?)?;
        }

        Ok(accumulator)
    }
}

fn combine_or_elements(
    left: &Node,
    right: &Node,
) -> Result<Node, EvaluateNodeError> {
    match (left, right) {
        (Node::Boolean(left), Node::Boolean(right)) => {
            if !cfg!(feature = "operator_or_boolean_boolean") {
                return Err(EvaluateNodeError::unsupported_operation());
            }

            Ok(Boolean::new(left.value || right.value)
                .with_tracable((left, right)))
        }
        (left, right) => Err(EvaluateNodeError::unsupported_operation()
            .with_tracable((left, right))),
    }
    .map(|node| node.with_tracable((left, right)))
}

#[cfg(test)]
mod tests {
    use node::NodeType;

    use super::*;

    #[test]
    fn evaluate_invalid_or() {
        let context = Context::default();
        let result = Or::new(vec![Boolean::new(true)]).evaluate(&context);
        assert_eq!(result, Err(EvaluateNodeError::invalid_node(NodeType::Or)));
    }

    #[test]
    fn evaluate_or_boolean_boolean_2() {
        let context = Context::default();
        let result = Or::new(vec![Boolean::new(false), Boolean::new(true)])
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Boolean::new(true));
    }

    #[test]
    fn evaluate_or_boolean_boolean_3() {
        let context = Context::default();
        let result = Or::new(vec![
            Boolean::new(false),
            Boolean::new(false),
            Boolean::new(false),
        ])
        .evaluate(&context)
        .unwrap();
        assert_eq!(result, Boolean::new(false));
    }

    #[test]
    fn evaluate_or_with_trace() {
        let context = Context::default();
        let result = Or::new(vec![
            Boolean::new(false).with_span(0, 0),
            Boolean::new(false).with_span(2, 2),
        ])
        .with_span(0, 2)
        .evaluate(&context)
        .unwrap();
        assert_eq!(result, Boolean::new(false).with_span(0, 2));
    }
}

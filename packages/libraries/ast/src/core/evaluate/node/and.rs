use crate::core::evaluate::EvaluateNode;
use crate::{
    And, Boolean, EvaluateNodeContext, EvaluateNodeError, GetNodeType, Node,
};
use trace::TracableMut;

impl EvaluateNode for And {
    fn evaluate(
        &self,
        context: &EvaluateNodeContext,
    ) -> Result<Node, EvaluateNodeError> {
        if self.elements.len() < 2 {
            return Err(EvaluateNodeError::invalid_node(self.node_type()));
        }

        self.elements
            .iter()
            .map(|element| element.evaluate(context))
            .reduce(|left, right| combine_and_elements(&left?, &right?))
            .expect("min 2 elements")
    }
}

fn combine_and_elements(
    left: &Node,
    right: &Node,
) -> Result<Node, EvaluateNodeError> {
    match (left, right) {
        (Node::Boolean(left), Node::Boolean(right)) => {
            if !cfg!(feature = "operator_and_boolean_boolean") {
                return Err(EvaluateNodeError::unsupported_operation());
            }

            Ok(Boolean::new(left.value && right.value)
                .with_tracable((left, right)))
        }
        (left, right) => Err(EvaluateNodeError::unsupported_operation()
            .with_tracable((left, right))),
    }
    .map(|node| node.with_tracable((left, right)))
}

#[cfg(test)]
mod tests {
    use crate::NodeType;

    use super::*;

    #[test]
    fn evaluate_invalid_and() {
        let context = EvaluateNodeContext::default();
        let result = And::new(vec![Boolean::new(true)]).evaluate(&context);
        assert_eq!(result, Err(EvaluateNodeError::invalid_node(NodeType::And)));
    }

    #[test]
    fn evaluate_and_boolean_boolean_2() {
        let context = EvaluateNodeContext::default();
        let result = And::new(vec![Boolean::new(false), Boolean::new(true)])
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Boolean::new(false));
    }

    #[test]
    fn evaluate_and_boolean_boolean_3() {
        let context = EvaluateNodeContext::default();
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
        let context = EvaluateNodeContext::default();
        let result = And::new(vec![
            Boolean::new(false).with_span(0, 0),
            Boolean::new(true).with_span(2, 2),
        ])
        .with_span(0, 2)
        .evaluate(&context)
        .unwrap();
        assert_eq!(result, Boolean::new(false).with_span(0, 2));
    }
}

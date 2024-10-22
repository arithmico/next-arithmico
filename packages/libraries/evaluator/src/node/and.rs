use crate::evaluate::EvaluateNode;
use ast::{And, Boolean, Node};
use common::{EvaluateNodeContext, EvaluateNodeError};
use trace::{IntoTrace, TracableMut};

impl EvaluateNode for And {
    fn evaluate(
        &self,
        context: &EvaluateNodeContext,
    ) -> Result<Node, EvaluateNodeError> {
        if self.elements.len() < 2 {
            return Err(EvaluateNodeError::InvalidNode);
        }

        let mut elements = self
            .elements
            .iter()
            .map(|element| element.evaluate(context));

        let mut accumulator = elements.next().unwrap()?;
        for current_element in elements {
            accumulator =
                combine_and_elements(&accumulator, &current_element?)?;
        }

        Ok(accumulator)
    }
}

fn combine_and_elements(
    left: &Node,
    right: &Node,
) -> Result<Node, EvaluateNodeError> {
    match (left, right) {
        (Node::Boolean(left), Node::Boolean(right)) => {
            if !cfg!(feature = "operator_and_boolean_boolean") {
                return Err(EvaluateNodeError::unsupported_operation((
                    left, right,
                )));
            }

            Ok(Boolean::new(left.value && right.value)
                .with_trace((left, right).into_trace()))
        }
        _ => Err(EvaluateNodeError::unsupported_operation((left, right))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_invalid_and() {
        let context = EvaluateNodeContext::default();
        let result = And::new(vec![Boolean::new(true)]).evaluate(&context);
        assert_eq!(result, Err(EvaluateNodeError::InvalidNode));
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
}

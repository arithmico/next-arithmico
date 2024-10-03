use ast::{Boolean, Node, Or};

use crate::{evaluate::EvaluateNode, Context, EvaluateNodeError};

impl EvaluateNode for Or {
    fn evaluate(&self, context: &Context) -> Result<Node, EvaluateNodeError> {
        if self.elements.len() < 2 {
            return Err(EvaluateNodeError::InvalidNode);
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
                return Err(EvaluateNodeError::UnsupportedOperation);
            }

            Ok(Boolean::new(left.value || right.value))
        }
        _ => Err(EvaluateNodeError::UnsupportedOperation),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_invalid_or() {
        let context = Context::default();
        let result = Or::new(vec![Boolean::new(true)]).evaluate(&context);
        assert_eq!(result, Err(EvaluateNodeError::InvalidNode));
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
}

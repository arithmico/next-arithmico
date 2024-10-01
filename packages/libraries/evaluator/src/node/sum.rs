use ast::{Node, Number, Sum};

use crate::{evaluate::EvaluateNode, Context, EvaluateNodeError};

impl EvaluateNode for Sum {
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
            accumulator = add_sum_elements(&accumulator, &current_element?)?;
        }

        Ok(accumulator)
    }
}

fn add_sum_elements(
    left: &Node,
    right: &Node,
) -> Result<Node, EvaluateNodeError> {
    match (left, right) {
        (Node::Number(left), Node::Number(right)) => {
            if !cfg!(feature = "datatype_number") {
                return Err(EvaluateNodeError::UnsupportedOperation);
            }

            Ok(Number::new(left.value + right.value))
        }
        _ => Err(EvaluateNodeError::UnsupportedOperation),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_invalid_sum() {
        let context = Context::default();
        let result = Sum::new(vec![Number::new(1.)]).evaluate(&context);
        assert_eq!(result, Err(EvaluateNodeError::InvalidNode));
    }

    #[test]
    fn evaluate_sum_number_number_2() {
        let context = Context::default();
        let result = Sum::new(vec![Number::new(1.), Number::new(2.)])
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Number::new(3.));
    }

    #[test]
    fn evaluate_sum_number_number_3() {
        let context = Context::default();
        let result =
            Sum::new(vec![Number::new(1.), Number::new(2.), Number::new(3.)])
                .evaluate(&context)
                .unwrap();
        assert_eq!(result, Number::new(6.));
    }
}

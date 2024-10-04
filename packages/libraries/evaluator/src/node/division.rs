use ast::{Division, Node, Number, Tensor};

use crate::{evaluate::EvaluateNode, EvaluateNodeContext, EvaluateNodeError};

impl EvaluateNode for Division {
    fn evaluate(&self, context: &EvaluateNodeContext) -> Result<Node, EvaluateNodeError> {
        let dividend = self.dividend.evaluate(context)?;
        let divisor = self.divisor.evaluate(context)?;

        match (dividend, divisor) {
            (Node::Number(dividend), Node::Number(divisor)) => {
                divide_number_by_number(&dividend, &divisor)
            }
            (Node::Tensor(dividend), Node::Number(divisor)) => {
                divide_tensor_by_number(&dividend, &divisor, context)
            }
            _ => Err(EvaluateNodeError::UnsupportedOperation),
        }
    }
}

fn divide_number_by_number(
    dividend: &Number,
    divisor: &Number,
) -> Result<Node, EvaluateNodeError> {
    if !cfg!(feature = "operator_division_number_number") {
        return Err(EvaluateNodeError::UnsupportedOperation);
    }

    if divisor.value == 0. {
        return Err(EvaluateNodeError::DivisionByZero);
    }

    Ok(Number::new(dividend.value / divisor.value))
}

fn divide_tensor_by_number(
    dividend: &Tensor,
    divisor: &Number,
    context: &EvaluateNodeContext,
) -> Result<Node, EvaluateNodeError> {
    if !cfg!(feature = "operator_division_tensor_number") {
        return Err(EvaluateNodeError::UnsupportedOperation);
    }

    let elements = dividend
        .elements
        .iter()
        .map(|element| {
            Division::new(element.clone(), Number::new(divisor.value))
        })
        .collect();

    Tensor::new_with_shape(dividend.shape.clone(), elements).evaluate(context)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_division_by_zero() {
        let context = EvaluateNodeContext::default();
        let result =
            Division::new(Number::new(8.), Number::new(0.)).evaluate(&context);
        assert_eq!(result, Err(EvaluateNodeError::DivisionByZero));
    }

    #[test]
    fn evaluate_division_number_number() {
        let context = EvaluateNodeContext::default();
        let result = Division::new(Number::new(8.), Number::new(2.))
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Number::new(4.));
    }

    #[test]
    fn evaluate_division_tensor_number() {
        let context = EvaluateNodeContext::default();
        let result = Division::new(
            Tensor::new(vec![
                Number::new(2.),
                Number::new(4.),
                Number::new(6.),
            ]),
            Number::new(2.),
        )
        .evaluate(&context)
        .unwrap();
        assert_eq!(
            result,
            Tensor::new(vec![
                Number::new(1.),
                Number::new(2.),
                Number::new(3.),
            ])
        );
    }
}

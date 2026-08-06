use node::{Division, Node, Number, Tensor};

use crate::core::{Context, EvaluateNode, EvaluateNodeError};

impl EvaluateNode for Division {
    fn evaluate(&self, context: &Context) -> Result<Node, EvaluateNodeError> {
        let dividend = self.dividend.evaluate(context)?;
        let divisor = self.divisor.evaluate(context)?;

        match (dividend, divisor) {
            (Node::Number(dividend), Node::Number(divisor)) => {
                divide_number_by_number(&dividend, &divisor)
            }
            (Node::Tensor(dividend), Node::Number(divisor)) => {
                divide_tensor_by_number(&dividend, &divisor, context)
            }
            _ => Err(EvaluateNodeError::unsupported_operation()),
        }
    }
}

fn divide_number_by_number(
    dividend: &Number,
    divisor: &Number,
) -> Result<Node, EvaluateNodeError> {
    if !cfg!(feature = "operator_division_number_number") {
        return Err(EvaluateNodeError::unsupported_operation());
    }

    if divisor.value == 0. {
        return Err(EvaluateNodeError::division_by_zero());
    }

    Ok(Number::new_node(dividend.value / divisor.value))
}

fn divide_tensor_by_number(
    dividend: &Tensor,
    divisor: &Number,
    context: &Context,
) -> Result<Node, EvaluateNodeError> {
    if !cfg!(feature = "operator_division_tensor_number") {
        return Err(EvaluateNodeError::unsupported_operation());
    }

    let elements = dividend
        .elements
        .iter()
        .map(|element| {
            Division::new(element.clone(), Number::new_node(divisor.value))
        })
        .collect();

    Tensor::new_with_shape(dividend.shape.clone(), elements).evaluate(context)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_division_by_zero() {
        let context = Context::default();
        let result = Division::new(Number::new_node(8.), Number::new_node(0.))
            .evaluate(&context);
        assert_eq!(result, Err(EvaluateNodeError::division_by_zero()));
    }

    #[test]
    fn evaluate_division_number_number() {
        let context = Context::default();
        let result = Division::new(Number::new_node(8.), Number::new_node(2.))
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Number::new_node(4.));
    }

    #[test]
    fn evaluate_division_tensor_number() {
        let context = Context::default();
        let result = Division::new(
            Tensor::new_node(vec![
                Number::new_node(2.),
                Number::new_node(4.),
                Number::new_node(6.),
            ]),
            Number::new_node(2.),
        )
        .evaluate(&context)
        .unwrap();
        assert_eq!(
            result,
            Tensor::new_node(vec![
                Number::new_node(1.),
                Number::new_node(2.),
                Number::new_node(3.),
            ])
        );
    }
}

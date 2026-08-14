use node::{Division, Node, Number, Tensor};

use crate::{Error, EvaluateNode, Options};

impl EvaluateNode for Division {
    fn evaluate(&self, options: Options) -> Result<Node, Error> {
        let dividend = self.dividend.evaluate(options)?;
        let divisor = self.divisor.evaluate(options)?;

        match (dividend, divisor) {
            (Node::Number(dividend), Node::Number(divisor)) => {
                divide_number_by_number(&dividend, &divisor)
            }
            (Node::Tensor(dividend), Node::Number(divisor)) => {
                divide_tensor_by_number(&dividend, &divisor, options)
            }
            _ => Err(Error::unsupported_operation()),
        }
    }
}

fn divide_number_by_number(
    dividend: &Number,
    divisor: &Number,
) -> Result<Node, Error> {
    if !cfg!(feature = "operator_division_number_number") {
        return Err(Error::unsupported_operation());
    }

    if divisor.value == 0. {
        return Err(Error::division_by_zero());
    }

    Ok(Number::new_node(dividend.value / divisor.value))
}

fn divide_tensor_by_number(
    dividend: &Tensor,
    divisor: &Number,
    options: Options,
) -> Result<Node, Error> {
    if !cfg!(feature = "operator_division_tensor_number") {
        return Err(Error::unsupported_operation());
    }

    let elements = dividend
        .elements
        .iter()
        .map(|element| {
            Division::new(element.clone(), Number::new_node(divisor.value))
        })
        .collect();

    Tensor::new_with_shape(dividend.shape.clone(), elements).evaluate(options)
}

#[cfg(test)]
mod tests {
    use crate::{Api, Stack};

    use super::*;

    #[test]
    fn evaluate_division_by_zero() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = Division::new(Number::new_node(8.), Number::new_node(0.))
            .evaluate(options);
        assert_eq!(result, Err(Error::division_by_zero()));
    }

    #[test]
    fn evaluate_division_number_number() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = Division::new(Number::new_node(8.), Number::new_node(2.))
            .evaluate(options)
            .unwrap();
        assert_eq!(result, Number::new_node(4.));
    }

    #[test]
    fn evaluate_division_tensor_number() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = Division::new(
            Tensor::new_node(vec![
                Number::new_node(2.),
                Number::new_node(4.),
                Number::new_node(6.),
            ]),
            Number::new_node(2.),
        )
        .evaluate(options)
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

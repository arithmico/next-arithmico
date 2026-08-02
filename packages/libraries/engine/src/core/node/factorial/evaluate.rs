use std::collections::VecDeque;

use float_utils::F64Extension;
use node::{Factorial, Node, Number};

use crate::core::{Context, EvaluateNode, EvaluateNodeError};

impl EvaluateNode for Factorial {
    fn evaluate(&self, context: &Context) -> Result<Node, EvaluateNodeError> {
        let value = self.value.evaluate(context)?;

        match value {
            Node::Number(number)
                if cfg!(feature = "operator_factorial_number") =>
            {
                if !number.value.is_integer() {
                    return Err(EvaluateNodeError::unsupported_operation());
                }
                if number.value < 0.0 {
                    return Err(EvaluateNodeError::unsupported_operation());
                }
                if number.value.is_close_to_zero() {
                    return Ok(Number::new_node(1.0));
                }

                let n = number.value.round() as usize;
                let mut queue = VecDeque::with_capacity(n);
                for i in 1..=n {
                    queue.push_back(i as f64);
                }

                loop {
                    match (queue.pop_front(), queue.pop_front()) {
                        (None, None) => unreachable!(),
                        (Some(v), None) | (None, Some(v)) => {
                            return Ok(Number::new_node(v));
                        }
                        (Some(a), Some(b)) => queue.push_back(a * b),
                    }
                }
            }
            _ => Err(EvaluateNodeError::unsupported_operation()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_factorial_number_err_not_int() {
        let context = Context::default();
        let result = Factorial::new(Number::new_node(0.5))
            .evaluate(&context)
            .unwrap_err();
        assert_eq!(result, EvaluateNodeError::unsupported_operation());
    }

    #[test]
    fn evaluate_factorial_number_err_negative_value() {
        let context = Context::default();
        let result = Factorial::new(Number::new_node(-1.0))
            .evaluate(&context)
            .unwrap_err();
        assert_eq!(result, EvaluateNodeError::unsupported_operation());
    }

    #[test]
    fn evaluate_factorial_number_0() {
        let context = Context::default();
        let result = Factorial::new(Number::new_node(0.0))
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Number::new_node(1.0));
    }

    #[test]
    fn evaluate_factorial_number_1() {
        let context = Context::default();
        let result = Factorial::new(Number::new_node(1.0))
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Number::new_node(1.0));
    }

    #[test]
    fn evaluate_factorial_number_2() {
        let context = Context::default();
        let result = Factorial::new(Number::new_node(2.0))
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Number::new_node(2.0));
    }

    #[test]
    fn evaluate_factorial_number_10() {
        let context = Context::default();
        let result = Factorial::new(Number::new_node(10.0))
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Number::new_node(3_628_800.0));
    }

    #[test]
    fn evaluate_factorial_number_20() {
        let context = Context::default();
        let result = Factorial::new(Number::new_node(20.0))
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Number::new_node(2_432_902_008_176_640_000.0));
    }
}

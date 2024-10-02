use ast::{Node, Number, Power};

use crate::{evaluate::EvaluateNode, Context, EvaluateNodeError};

impl EvaluateNode for Power {
    fn evaluate(&self, context: &Context) -> Result<Node, EvaluateNodeError> {
        if !cfg!(feature = "operator_power_number_number") {
            return Err(EvaluateNodeError::UnsupportedOperation);
        }

        let base = self.base.evaluate(context)?;
        let exponent = self.exponent.evaluate(context)?;

        match (base, exponent) {
            (Node::Number(base), Node::Number(exponent)) => {
                if exponent.value == 0. {
                    return Ok(Number::new(1.));
                }

                if base.value == 0. {
                    return Ok(Number::new(0.));
                }

                Ok(Number::new(base.value.powf(exponent.value)))
            }
            _ => Err(EvaluateNodeError::UnsupportedOperation),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_power_number_number() {
        let context = Context::default();
        let result = Power::new(Number::new(8.), Number::new(2.))
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Number::new(64.));
    }

    #[test]
    fn evaluate_power_number_number0() {
        let context = Context::default();
        let result = Power::new(Number::new(8.), Number::new(0.))
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Number::new(1.));
    }

    #[test]
    fn evaluate_power_number0_number() {
        let context = Context::default();
        let result = Power::new(Number::new(0.), Number::new(2.))
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Number::new(0.));
    }
}

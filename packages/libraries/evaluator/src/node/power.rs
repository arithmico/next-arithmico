use ast::{Node, Number, Power};
use common::{EvaluateNodeContext, EvaluateNodeError};

use crate::evaluate::EvaluateNode;

impl EvaluateNode for Power {
    fn evaluate(
        &self,
        context: &EvaluateNodeContext,
    ) -> Result<Node, EvaluateNodeError> {
        let base = self.base.evaluate(context)?;
        let exponent = self.exponent.evaluate(context)?;

        match (base, exponent) {
            (Node::Number(base), Node::Number(exponent)) => {
                if !cfg!(feature = "operator_power_number_number") {
                    return Err(EvaluateNodeError::unsupported_operation());
                }

                if exponent.value == 0. {
                    return Ok(Number::new(1.));
                }

                if base.value == 0. {
                    return Ok(Number::new(0.));
                }

                Ok(Number::new(base.value.powf(exponent.value)))
            }
            (base, exponent) => {
                Err(EvaluateNodeError::unsupported_operation())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use trace::TracableMut;

    #[test]
    fn evaluate_power_number_number() {
        let context = EvaluateNodeContext::default();
        let result = Power::new(Number::new(8.), Number::new(2.))
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Number::new(64.));
    }

    #[test]
    fn evaluate_power_number_number_with_trace() {
        let context = EvaluateNodeContext::default();
        let result = Power::new(
            Number::new(8.).with_span(0, 0),
            Number::new(2.).with_span(2, 2),
        )
        .with_span(0, 2)
        .evaluate(&context)
        .unwrap();
        assert_eq!(result, Number::new(64.).with_span(0, 2));
    }

    #[test]
    fn evaluate_power_number_number0() {
        let context = EvaluateNodeContext::default();
        let result = Power::new(Number::new(8.), Number::new(0.))
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Number::new(1.));
    }

    #[test]
    fn evaluate_power_number_number0_with_trace() {
        let context = EvaluateNodeContext::default();
        let result = Power::new(
            Number::new(8.).with_span(0, 0),
            Number::new(0.).with_span(2, 2),
        )
        .with_span(0, 2)
        .evaluate(&context)
        .unwrap();
        assert_eq!(result, Number::new(1.).with_span(0, 2));
    }

    #[test]
    fn evaluate_power_number0_number() {
        let context = EvaluateNodeContext::default();
        let result = Power::new(Number::new(0.), Number::new(2.))
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Number::new(0.));
    }

    #[test]
    fn evaluate_power_number0_number_with_trace() {
        let context = EvaluateNodeContext::default();
        let result = Power::new(
            Number::new(0.).with_span(0, 0),
            Number::new(2.).with_span(2, 2),
        )
        .with_span(0, 2)
        .evaluate(&context)
        .unwrap();
        assert_eq!(result, Number::new(0.).with_span(0, 2));
    }
}

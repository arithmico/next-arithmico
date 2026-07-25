use node::{Node, Number, Power};
use trace::{Tracable, TracableMut};

use crate::core::{Context, EvaluateNode, EvaluateNodeError};

impl EvaluateNode for Power {
    fn evaluate(&self, context: &Context) -> Result<Node, EvaluateNodeError> {
        let base = self.base.evaluate(context)?;
        let exponent = self.exponent.evaluate(context)?;

        match (&base, &exponent) {
            (Node::Number(base), Node::Number(exponent))
                if cfg!(feature = "operator_power_number_number") =>
            {
                if exponent.value == 0. {
                    return Ok(Number::new_node(1.));
                }

                if base.value == 0. {
                    return Ok(Number::new_node(0.));
                }

                Ok(Number::new_node(base.value.powf(exponent.value)))
            }
            _ => Err(EvaluateNodeError::unsupported_operation()
                .with_optional_span(base.hull())
                .with_optional_span(exponent.hull())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lexer::Span;
    use trace::TracableMut;

    #[test]
    fn evaluate_power_number_number() {
        let context = Context::default();
        let result = Power::new(Number::new_node(8.), Number::new_node(2.))
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Number::new_node(64.));
    }

    #[test]
    fn evaluate_power_number_number_with_trace() {
        let context = Context::default();
        let result = Power::new(
            Number::new_node(8.).with_span(Span::new_between(0, 0)),
            Number::new_node(2.).with_span(Span::new_between(2, 2)),
        )
        .with_span(Span::new_between(0, 2))
        .evaluate(&context)
        .unwrap();
        assert_eq!(
            result,
            Number::new_node(64.).with_span(Span::new_between(0, 2))
        );
    }

    #[test]
    fn evaluate_power_number_number0() {
        let context = Context::default();
        let result = Power::new(Number::new_node(8.), Number::new_node(0.))
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Number::new_node(1.));
    }

    #[test]
    fn evaluate_power_number_number0_with_trace() {
        let context = Context::default();
        let result = Power::new(
            Number::new_node(8.).with_span(Span::new_between(0, 0)),
            Number::new_node(0.).with_span(Span::new_between(2, 2)),
        )
        .with_span(Span::new_between(0, 2))
        .evaluate(&context)
        .unwrap();
        assert_eq!(
            result,
            Number::new_node(1.).with_span(Span::new_between(0, 2))
        );
    }

    #[test]
    fn evaluate_power_number0_number() {
        let context = Context::default();
        let result = Power::new(Number::new_node(0.), Number::new_node(2.))
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Number::new_node(0.));
    }

    #[test]
    fn evaluate_power_number0_number_with_trace() {
        let context = Context::default();
        let result = Power::new(
            Number::new_node(0.).with_span(Span::new_between(0, 0)),
            Number::new_node(2.).with_span(Span::new_between(2, 2)),
        )
        .with_span(Span::new_between(0, 2))
        .evaluate(&context)
        .unwrap();
        assert_eq!(
            result,
            Number::new_node(0.).with_span(Span::new_between(0, 2))
        );
    }
}

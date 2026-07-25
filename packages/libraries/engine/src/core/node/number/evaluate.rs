use node::{GetNodeType, Node, Number};

use crate::core::{Context, EvaluateNode, EvaluateNodeError};

impl EvaluateNode for Number {
    fn evaluate(&self, _context: &Context) -> Result<Node, EvaluateNodeError> {
        if !cfg!(feature = "datatype_number") {
            return Err(EvaluateNodeError::unsupported_datatype(
                self.node_type(),
            ));
        }

        Ok(Number::new_node(self.value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lexer::Span;
    use trace::TracableMut;

    #[test]
    fn evaluate_number() {
        let context = Context::default();
        let result = Number::new_node(1.2345).evaluate(&context).unwrap();
        assert_eq!(result, Number::new_node(1.2345));
    }

    #[test]
    fn evaluate_number_with_trace() {
        let context = Context::default();
        let result = Number::new_node(1.2345)
            .with_span(Span::new_between(0, 5))
            .evaluate(&context)
            .unwrap();
        assert_eq!(
            result,
            Number::new_node(1.2345).with_span(Span::new_between(0, 5))
        );
    }
}

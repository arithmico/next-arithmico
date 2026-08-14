use node::{GetNodeType, Node, Number};

use crate::{Error, EvaluateNode, Options};

impl EvaluateNode for Number {
    fn evaluate(&self, _context: Options) -> Result<Node, Error> {
        if !cfg!(feature = "datatype_number") {
            return Err(Error::unsupported_datatype(self.node_type()));
        }

        Ok(Number::new_node(self.value))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Api, Stack};

    use super::*;
    use lexer::Span;
    use trace::TracableMut;

    #[test]
    fn evaluate_number() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = Number::new_node(1.2345).evaluate(options).unwrap();
        assert_eq!(result, Number::new_node(1.2345));
    }

    #[test]
    fn evaluate_number_with_trace() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = Number::new_node(1.2345)
            .with_span(Span::new_between(0, 5))
            .evaluate(options)
            .unwrap();
        assert_eq!(
            result,
            Number::new_node(1.2345).with_span(Span::new_between(0, 5))
        );
    }
}

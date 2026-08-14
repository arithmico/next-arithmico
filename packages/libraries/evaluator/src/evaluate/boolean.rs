use node::{Boolean, GetNodeType, Node};
use trace::{Tracable, TracableMut};

use crate::{Error, EvaluateNode, Options};

impl EvaluateNode for Boolean {
    fn evaluate(&self, _context: Options) -> Result<Node, Error> {
        if !cfg!(feature = "datatype_boolean") {
            return Err(Error::unsupported_datatype(self.node_type()));
        }

        Ok(Boolean::new(self.value).with_optional_span(self.hull()))
    }
}

#[cfg(test)]
mod tests {
    use lexer::Span;
    use trace::TracableMut;

    use crate::{Api, Stack};

    use super::*;

    #[test]
    fn evaluate_boolean_true() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = Boolean::new(true).evaluate(options).unwrap();
        assert_eq!(result, Boolean::new(true));
    }

    #[test]
    fn evaluate_boolean_false() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = Boolean::new(false).evaluate(options).unwrap();
        assert_eq!(result, Boolean::new(false));
    }

    #[test]
    fn evaluate_boolean_with_trace() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = Boolean::new(false)
            .with_span(Span::new_between(0, 4))
            .evaluate(options)
            .unwrap();
        assert_eq!(
            result,
            Boolean::new(false).with_span(Span::new_between(0, 4))
        );
    }
}

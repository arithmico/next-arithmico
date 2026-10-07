use node::{Boolean, GetNodeType, Node, Or};
use trace::{Tracable, TracableMut};

use crate::{Error, EvaluateNode, Options};

impl EvaluateNode for Or {
    fn evaluate(&self, context: Options) -> Result<Node, Error> {
        if self.elements.len() < 2 {
            return Err(Error::invalid_node(self.node_type()));
        }

        let mut elements = self
            .elements
            .iter()
            .map(|element| element.evaluate(context));

        let mut accumulator = elements
            .next()
            .ok_or_else(|| Error::invalid_node(self.node_type()))??;
        for current_element in elements {
            accumulator = combine_or_elements(&accumulator, &current_element?)?;
        }

        Ok(accumulator)
    }
}

fn combine_or_elements(left: &Node, right: &Node) -> Result<Node, Error> {
    match (left, right) {
        (Node::Boolean(left), Node::Boolean(right))
            if cfg!(feature = "operator_or_boolean_boolean") =>
        {
            Ok(Boolean::new(left.value || right.value)
                .with_optional_span(left.hull())
                .with_optional_span(right.hull()))
        }
        (left, right) => Err(Error::unsupported_operation()
            .with_optional_span(left.hull())
            .with_optional_span(right.hull())),
    }
}

#[cfg(test)]
mod tests {
    use lexer::Span;
    use node::NodeType;

    use crate::{Api, Stack};

    use super::*;

    #[test]
    fn evaluate_invalid_or() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = Or::new(vec![Boolean::new(true)]).evaluate(options);
        assert_eq!(result, Err(Error::invalid_node(NodeType::Or)));
    }

    #[test]
    fn evaluate_or_boolean_boolean_2() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = Or::new(vec![Boolean::new(false), Boolean::new(true)])
            .evaluate(options)
            .unwrap();
        assert_eq!(result, Boolean::new(true));
    }

    #[test]
    fn evaluate_or_boolean_boolean_3() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = Or::new(vec![
            Boolean::new(false),
            Boolean::new(false),
            Boolean::new(false),
        ])
        .evaluate(options)
        .unwrap();
        assert_eq!(result, Boolean::new(false));
    }

    #[test]
    fn evaluate_or_with_trace() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = Or::new(vec![
            Boolean::new(false).with_span(Span::new_between(0, 0)),
            Boolean::new(false).with_span(Span::new_between(2, 2)),
        ])
        .with_span(Span::new_between(0, 2))
        .evaluate(options)
        .unwrap();
        assert_eq!(
            result,
            Boolean::new(false).with_span(Span::new_between(0, 2))
        );
    }
}

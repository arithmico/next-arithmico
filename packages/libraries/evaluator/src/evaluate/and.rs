use node::{And, Boolean, GetNodeType, Node};
use trace::{CombineHulls, Tracable, TracableMut};

use crate::{Error, EvaluateNode, Options};

impl EvaluateNode for And {
    fn evaluate(&self, context: Options) -> Result<Node, Error> {
        if self.elements.len() < 2 {
            return Err(Error::invalid_node(self.node_type()));
        }

        self.elements
            .iter()
            .map(|element| element.evaluate(context))
            .reduce(|left, right| combine_and_elements(&left?, &right?))
            .ok_or_else(Error::unreachable)?
    }
}

fn combine_and_elements(left: &Node, right: &Node) -> Result<Node, Error> {
    match (left, right) {
        (Node::Boolean(left), Node::Boolean(right))
            if cfg!(feature = "operator_and_boolean_boolean") =>
        {
            let span = (left, right).combine_hulls();
            Ok(
                Boolean::new(left.value && right.value)
                    .with_optional_span(span),
            )
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
    fn evaluate_invalid_and() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = And::new(vec![Boolean::new(true)]).evaluate(options);
        assert_eq!(result, Err(Error::invalid_node(NodeType::And)));
    }

    #[test]
    fn evaluate_and_boolean_boolean_2() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = And::new(vec![Boolean::new(false), Boolean::new(true)])
            .evaluate(options)
            .unwrap();
        assert_eq!(result, Boolean::new(false));
    }

    #[test]
    fn evaluate_and_boolean_boolean_3() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = And::new(vec![
            Boolean::new(true),
            Boolean::new(true),
            Boolean::new(true),
        ])
        .evaluate(options)
        .unwrap();
        assert_eq!(result, Boolean::new(true));
    }

    #[test]
    fn evaluate_and_with_trace() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = And::new(vec![
            Boolean::new(false).with_span(Span::new_between(0, 0)),
            Boolean::new(true).with_span(Span::new_between(2, 2)),
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

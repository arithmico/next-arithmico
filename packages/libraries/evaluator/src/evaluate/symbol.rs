use node::{GetNodeType, Node, Symbol};
use trace::{Tracable, TracableMut};

use crate::{Error, EvaluateNode, Options};

impl EvaluateNode for Symbol {
    fn evaluate(&self, context: Options) -> Result<Node, Error> {
        if !cfg!(feature = "datatype_symbol") {
            return Err(Error::unsupported_datatype(self.node_type()));
        }

        context.lookup(&self.name).ok_or_else(|| {
            Error::unknown_symbol(&self.name).with_optional_span(self.hull())
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{Api, ApiModule, Stack};

    use super::*;
    use lexer::Span;
    use node::Number;
    use trace::TracableMut;
    use translate_core::Language;

    #[test]
    fn evaluate_unknown_symbol() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = Symbol::new("x").evaluate(options);
        assert_eq!(result, Err(Error::unknown_symbol("x")));
    }

    #[test]
    fn evaluate_symbol_from_stack() {
        let mut stack = Stack::new();
        stack.insert("x", Number::new_node(42.));
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = Symbol::new("x").evaluate(options).unwrap();
        assert_eq!(result, Number::new_node(42.));
    }

    #[test]
    fn evaluate_symbol_from_stack_with_trace() {
        let mut stack = Stack::new();
        stack.insert("x", Number::new_node(42.));
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = Symbol::new("x")
            .with_span(Span::new_between(0, 0))
            .evaluate(options)
            .unwrap();
        assert_eq!(
            result,
            Number::new_node(42.).with_span(Span::new_between(0, 0))
        );
    }

    #[test]
    fn evaluate_symbol_from_host_api() {
        let stack = Stack::new();
        let api = Api::builder()
            .module(true, || {
                ApiModule::builder()
                    .id("test")
                    .name(Language::English, "test")
                    .endpoints(&[|builder| {
                        builder
                            .name("test")
                            .description(Language::English, "test")
                            .constant(|_options| Number::new_node(42.0).into())
                    }])
                    .build()
            })
            .build();

        let options = Options::new(&stack, &api, common::AngleUnit::Radian);

        let result = Symbol::new("test").evaluate(options).unwrap();
        assert_eq!(result, Number::new_node(42.));
    }

    #[test]
    fn evaluate_symbol_from_host_api_with_trace() {
        let stack = Stack::new();
        let api = Api::builder()
            .module(true, || {
                ApiModule::builder()
                    .id("test")
                    .name(Language::English, "test")
                    .endpoints(&[|builder| {
                        builder
                            .name("test")
                            .description(Language::English, "test")
                            .constant(|_options| Number::new_node(42.0).into())
                    }])
                    .build()
            })
            .build();

        let options = Options::new(&stack, &api, common::AngleUnit::Radian);

        let result = Symbol::new("test")
            .with_span(Span::new_between(0, 3))
            .evaluate(options)
            .unwrap();
        assert_eq!(
            result,
            Number::new_node(42.).with_span(Span::new_between(0, 3))
        );
    }
}

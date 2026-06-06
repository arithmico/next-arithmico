use crate::core::{
    Context, EvaluateNode, EvaluateNodeError, GetNodeType, Node, Symbol,
};

impl EvaluateNode for Symbol {
    fn evaluate(&self, context: &Context) -> Result<Node, EvaluateNodeError> {
        if !cfg!(feature = "datatype_symbol") {
            return Err(EvaluateNodeError::unsupported_datatype(
                self.node_type(),
            ));
        }

        context
            .lookup(&self.name)
            .ok_or_else(|| EvaluateNodeError::unknown_symbol(&self.name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        DecimalFormat, DecimalPlaces,
        core::{HostApi, HostApiModule, Language, Number, Stack},
    };
    use std::sync::Arc;
    use trace::TracableMut;

    #[test]
    fn evaluate_unknown_symbol() {
        let context = Context::default();
        let result = Symbol::new("x").evaluate(&context);
        assert_eq!(result, Err(EvaluateNodeError::unknown_symbol("x")));
    }

    #[test]
    fn evaluate_symbol_from_stack() {
        let mut stack = Stack::new();
        stack.insert("x", Number::new(42.));
        let context = Context::new(
            stack,
            DecimalPlaces::default(),
            DecimalFormat::default(),
            Arc::new(HostApi::empty()),
        );
        let result = Symbol::new("x").evaluate(&context).unwrap();
        assert_eq!(result, Number::new(42.));
    }

    #[test]
    fn evaluate_symbol_from_stack_with_trace() {
        let mut stack = Stack::new();
        stack.insert("x", Number::new(42.));
        let context = Context::new(
            stack,
            DecimalPlaces::default(),
            DecimalFormat::default(),
            Arc::new(HostApi::empty()),
        );
        let result =
            Symbol::new("x").with_span(0, 0).evaluate(&context).unwrap();
        assert_eq!(result, Number::new(42.).with_span(0, 0));
    }

    #[test]
    fn evaluate_symbol_from_host_api() {
        let host_api = HostApi::builder()
            .module(true, || {
                HostApiModule::builder()
                    .id("test")
                    .name(Language::English, "test")
                    .endpoint(true, "test", |builder| {
                        builder
                            .description(Language::English, "test")
                            .constant(|_context| Number::new(42.0).into())
                    })
                    .build()
            })
            .build();

        let context = Context::new(
            Stack::new(),
            DecimalPlaces::default(),
            DecimalFormat::default(),
            host_api.into(),
        );

        let result = Symbol::new("test").evaluate(&context).unwrap();
        assert_eq!(result, Number::new(42.));
    }

    #[test]
    fn evaluate_symbol_from_host_api_with_trace() {
        let host_api = HostApi::builder()
            .module(true, || {
                HostApiModule::builder()
                    .id("test")
                    .name(Language::English, "test")
                    .endpoint(true, "test", |builder| {
                        builder
                            .description(Language::English, "test")
                            .constant(|_context| Number::new(42.0).into())
                    })
                    .build()
            })
            .build();

        let context = Context::new(
            Stack::new(),
            DecimalPlaces::default(),
            DecimalFormat::default(),
            host_api.into(),
        );

        let result = Symbol::new("test")
            .with_span(0, 3)
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Number::new(42.).with_span(0, 3));
    }
}

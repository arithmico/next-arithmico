use ast::{Node, Symbol};
use common::{EvaluateNodeContext, EvaluateNodeError};

use crate::evaluate::EvaluateNode;

impl EvaluateNode for Symbol {
    fn evaluate(
        &self,
        context: &EvaluateNodeContext,
    ) -> Result<Node, EvaluateNodeError> {
        if !cfg!(feature = "datatype_symbol") {
            return Err(EvaluateNodeError::UnsupportedDataType(String::from(
                "symbol",
            )));
        }

        context
            .lookup(&self.name)
            .ok_or_else(|| EvaluateNodeError::UnknownSymbol(self.name.clone()))
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use ast::Number;
    use common::{
        EvaluateNodeOptions, HostApi, HostApiModule, Language, Stack,
    };

    use super::*;

    #[test]
    fn evaluate_unknown_symbol() {
        let context = EvaluateNodeContext::default();
        let result = Symbol::new("x").evaluate(&context);
        assert_eq!(result, Err(EvaluateNodeError::UnknownSymbol("x".into())));
    }

    #[test]
    fn evaluate_symbol_from_stack() {
        let mut stack = Stack::new();
        stack.insert("x", Number::new(42.));
        let context = EvaluateNodeContext::new(
            stack,
            EvaluateNodeOptions::default(),
            Rc::new(HostApi::empty()),
        );
        let result = Symbol::new("x").evaluate(&context).unwrap();
        assert_eq!(result, Number::new(42.));
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

        let context = EvaluateNodeContext::new(
            Stack::new(),
            EvaluateNodeOptions::default(),
            host_api.into(),
        );

        let result = Symbol::new("test").evaluate(&context).unwrap();
        assert_eq!(result, Number::new(42.));
    }
}

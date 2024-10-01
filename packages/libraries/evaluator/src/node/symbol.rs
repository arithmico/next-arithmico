use ast::{Node, Symbol};

use crate::{evaluate::EvaluateNode, Context, EvaluateNodeError};

impl EvaluateNode for Symbol {
    fn evaluate(&self, context: &Context) -> Result<Node, EvaluateNodeError> {
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
    use common::Language;

    use crate::context::{HostApi, HostApiModule, Settings, Stack};

    use super::*;

    #[test]
    fn evaluate_unknown_symbol() {
        let context = Context::default();
        let result = Symbol::new("x").evaluate(&context);
        assert_eq!(result, Err(EvaluateNodeError::UnknownSymbol("x".into())));
    }

    #[test]
    fn evaluate_symbol_from_stack() {
        let mut stack = Stack::new();
        stack.insert("x", Number::new(42.));
        let context =
            Context::new(stack, Settings::default(), Rc::new(HostApi::empty()));
        let result = Symbol::new("x").evaluate(&context).unwrap();
        assert_eq!(result, Number::new(42.));
    }

    #[test]
    fn evaluate_symbol_from_host_api() {
        let host_api = HostApi::builder()
            .module(true, || {
                HostApiModule::builder()
                    .name("test")
                    .endpoint(true, "test", |builder| {
                        builder
                            .description(Language::English, "test")
                            .constant(|_context| Number::new(42.0).into())
                    })
                    .build()
            })
            .build();

        let context =
            Context::new(Stack::new(), Settings::default(), host_api.into());

        let result = Symbol::new("test").evaluate(&context).unwrap();
        assert_eq!(result, Number::new(42.));
    }
}

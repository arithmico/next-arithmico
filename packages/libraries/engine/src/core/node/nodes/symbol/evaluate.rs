use crate::core::{context::Context, node::*};

impl EvaluateNode for Symbol {
    fn evaluate(&self, context: &Context) -> Result<Node, NodeError> {
        context
            .lookup(&self.name)
            .ok_or_else(|| NodeError::RuntimeError("unkown symbol".into()))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        core::{context::Stack, host_api::HostApiModule},
        language::Language,
        HostApi, Settings,
    };

    use super::*;

    #[test]
    fn evaluate_symbol() {
        let host_api = HostApi::builder().build();
        let mut stack = Stack::new();
        stack.insert("test", Number::new(36.0).into());
        let context = Context::new(stack, Settings::default(), host_api.into());

        assert_eq!(
            Node::from(Symbol::new("test")).evaluate(&context).unwrap(),
            Number::new(36.0).into()
        )
    }

    #[test]
    fn evaluate_host_constant_endpoint() {
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

        assert_eq!(
            Node::from(Symbol::new("test")).evaluate(&context).unwrap(),
            Number::new(42.0).into()
        )
    }
}

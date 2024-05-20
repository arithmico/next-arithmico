use crate::core::{
    context::Context, evaluate::NodeEvaluationError, node::Node,
};

pub fn evaluate_symbol(
    name: &String,
    context: &Context,
) -> Result<Node, NodeEvaluationError> {
    context
        .lookup(name)
        .ok_or_else(|| NodeEvaluationError::UnknownSymbol)
}

#[cfg(test)]
mod tests {
    use crate::{
        core::context::{HostApiModule, Stack},
        HostApi, Settings,
    };

    use super::*;

    #[test]
    fn evaluate_symbol() {
        let host_api = HostApi::builder().build();
        let mut stack = Stack::new();
        stack.insert("test", Node::Number { value: 36.0 });
        let context = Context::new(stack, Settings::default(), host_api.into());

        assert_eq!(
            Node::Symbol {
                name: String::from("test")
            }
            .evaluate(&context)
            .unwrap(),
            Node::Number { value: 36.0 }
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
                            .description(crate::Language::English, "test")
                            .constant(|_context| Node::Number { value: 42.0 })
                    })
                    .build()
            })
            .build();

        let context =
            Context::new(Stack::new(), Settings::default(), host_api.into());

        assert_eq!(
            Node::Symbol {
                name: "test".into()
            }
            .evaluate(&context)
            .unwrap(),
            Node::Number { value: 42.0 }
        )
    }
}

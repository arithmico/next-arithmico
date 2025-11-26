use leptos::prelude::*;

use engine::{Context, Node, NodeConverter, Serialize, SessionError};

use crate::components::CalculatorErrorOutput;

#[component]
pub fn CalculatorOutput(
    #[prop(into)] value: Signal<Option<Result<Node, SessionError>>>,
    #[prop(into)] context: Signal<Context>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <output
            for="calculator-output"
            data-testid="calculator-output"
            class=format!(
                "calculator-output {}",
                class.unwrap_or(String::new()),
            )
        >
            {move || {
                let context = context.get();
                match value.get() {
                    Some(result) => {
                        match result {
                            Ok(node) => {
                                NodeToStringConverter::new(context).convert_node(&node)
                            }
                            Err(error) => {
                                view! { <CalculatorErrorOutput error=error /> }.into_any()
                            }
                        }
                    }
                    None => " ".into_any(),
                }
            }}
        </output>
    }
}

struct NodeToStringConverter {
    context: Context,
}

impl NodeToStringConverter {
    pub fn new(context: Context) -> Self {
        Self { context }
    }
}

impl NodeConverter<AnyView> for NodeToStringConverter {
    fn convert_node(&self, node: &Node) -> AnyView {
        node.serialize(&self.context).into_any()
    }
}

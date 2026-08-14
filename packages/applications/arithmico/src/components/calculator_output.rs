use leptos::prelude::*;

use engine::{SerializeNode, SerializeOptions, SessionError};
use node::Node;

use crate::components::CalculatorErrorOutput;

#[component]
pub fn CalculatorOutput(
    #[prop(into)] value: Signal<Option<Result<Node, SessionError>>>,
    #[prop(into)] options: Signal<SerializeOptions>,
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
                let options = options.get();
                match value.get() {
                    Some(result) => {
                        match result {
                            Ok(node) => {
                                node.serialize(options)
                                    .unwrap_or_else(|_| { String::from("SerializationError") })
                                    .into_any()
                            }
                            Err(error) => {
                                // TODO: translate serialization error
                                view! { <CalculatorErrorOutput error=error /> }
                                    .into_any()
                            }
                        }
                    }
                    None => view! { <>" "</> }.into_any(),
                }
            }}
        </output>
    }
}

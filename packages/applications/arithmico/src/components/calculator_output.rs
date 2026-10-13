use engine_protocol::Error;
use leptos::prelude::*;
use node::Node;

use crate::components::{CalculatorErrorOutput, DisplayNode};

#[component]
pub fn CalculatorOutput(
    #[prop(into)] value: Signal<Option<Result<Node, Error>>>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <output
            for="calculator-output"
            data-testid="calculator-output"
            class=format!("calculator-output {}", class.unwrap_or_default())
        >
            {move || {
                match value.get() {
                    Some(result) => {
                        match result {
                            Ok(node) => view! { <DisplayNode node=node /> }.into_any(),
                            Err(error) => {
                                view! { <CalculatorErrorOutput error=error /> }.into_any()
                            }
                        }
                    }
                    None => view! { <>" "</> }.into_any(),
                }
            }}
        </output>
    }
}

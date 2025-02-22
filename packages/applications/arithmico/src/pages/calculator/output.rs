use engine::SessionError;
use evaluate_node_error_output::EvaluateNodeErrorOutput;
use leptos::prelude::*;
use parse_node_error_output::ParseNodeErrorOutput;
use serialize_node_error_output::SerializeNodeErrorOutput;

mod evaluate_node_error_output;
mod parse_node_error_output;
mod serialize_node_error_output;

#[component]
pub fn CalculatorOutput(
    value: Signal<Option<Result<String, SessionError>>>,
) -> impl IntoView {
    view! {
        <output
            for="calculator-output"
            data-testid="calculator-output"
            class="calculator-output"
        >
            {move || match value.get() {
                Some(result) => {
                    match result {
                        Ok(value) => view! { <>{value}</> }.into_any(),
                        Err(error) => {
                            match error {
                                SessionError::ParseNodeError(_) => {
                                    view! { <ParseNodeErrorOutput /> }.into_any()
                                }
                                SessionError::SerializeNodeError(_) => {
                                    view! { <SerializeNodeErrorOutput /> }.into_any()
                                }
                                SessionError::EvaluateNodeError(error) => {
                                    view! { <EvaluateNodeErrorOutput value=error /> }.into_any()
                                }
                            }
                        }
                    }
                }
                None => view! { <>" "</> }.into_any(),
            }}

        </output>
    }
}

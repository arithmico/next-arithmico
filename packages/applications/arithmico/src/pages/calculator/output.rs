use engine::SessionError;
use evaluate_node_error_output::EvaluateNodeErrorOutput;
use leptos::*;
use parse_node_error_output::ParseNodeErrorOutput;
use serialize_node_error_output::SerializeNodeErrorOutput;

use crate::class_names;

mod evaluate_node_error_output;
mod parse_node_error_output;
mod serialize_node_error_output;

#[component]
pub fn OutputField(
    value: Signal<Option<Result<String, SessionError>>>,
) -> impl IntoView {
    view! {
        <output
            for="calculator-output"
            data-testid="calculator-output"
            class=class_names!(
                "p-2",
                "text-xl",
                "whitespace-pre-wrap",
                "rounded-sm",
                "border-2",
                "outline-none",
                "theme-light:text-black",
                "theme-dark:text-white",
                "theme-light:bg-white",
                "theme-dark:bg-neutral-800",
                "theme-light:border-neutral-300",
                "theme-dark:border-neutral-700"
            )
        >
            {move || match value.get() {
                Some(result) => {
                    match result {
                        Ok(value) => view! { <>{value}</> }.into_view(),
                        Err(error) => {
                            match error {
                                SessionError::ParseNodeError(error) => {
                                    view! { <ParseNodeErrorOutput value=error /> }.into_view()
                                }
                                SessionError::SerializeNodeError(error) => {
                                    view! { <SerializeNodeErrorOutput value=error /> }
                                        .into_view()
                                }
                                SessionError::EvaluateNodeError(error) => {
                                    view! { <EvaluateNodeErrorOutput value=error /> }
                                }
                            }
                        }
                    }
                }
                None => view! { <>" "</> }.into_view(),
            }}

        </output>
    }
}

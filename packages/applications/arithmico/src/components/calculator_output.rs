use leptos::prelude::*;

use engine::SessionError;

use crate::components::CalculatorErrorOutput;

#[component]
pub fn CalculatorOutput(
    #[prop(into)] value: Signal<Option<Result<String, SessionError>>>,
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
            {move || match value.get() {
                Some(result) => {
                    match result {
                        Ok(value) => view! { <>{value}</> }.into_any(),
                        Err(error) => {
                            view! { <CalculatorErrorOutput error=error /> }.into_any()
                        }
                    }
                }
                None => view! { <>" "</> }.into_any(),
            }}
        </output>
    }
}

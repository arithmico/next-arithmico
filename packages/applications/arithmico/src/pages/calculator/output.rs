use engine::SessionError;
use evaluate_node_error_output::EvaluateNodeErrorOutput;
use leptos::*;
use parse_node_error_output::ParseNodeErrorOutput;
use serialize_node_error_output::SerializeNodeErrorOutput;

mod evaluate_node_error_output;
mod parse_node_error_output;
mod serialize_node_error_output;

#[component]
pub fn OutputField(
    value: Signal<Option<Result<String, SessionError>>>,
) -> impl IntoView {
    /*let output = move || match value.get() {
        None => (String::new(), false),
        Some(result) => match result {
            Ok(result) => (result, false),
            Err(error) => (error.to_string(), true),
        },
    };
    let content = move || {
        let output = output().0;
        if output.is_empty() {
            // TODO: consider using css content = "\200b" instead of " "
            String::from(" ")
        } else {
            output
        }
    };*/
    //let is_error = move || output().1;
    view! {
        <output
            for="calculator-output"
            data-testid="calculator-output"
            class="p-2 text-xl whitespace-pre-wrap bg-white rounded-sm border outline-none focus-visible:border-black border-neutral-300"
        >
            // class:border-black=move || !is_error()
            // class:border-red-500=move || is_error()
            // class:bg-red-100=move || is_error()
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
                                    view! { <SerializeNodeErrorOutput value=error /> }.into_view()
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

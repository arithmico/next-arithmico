use crate::{
    components::*, pages::calculator::input_field::InputField,
    utils::expect_app_state,
};
use engine::SessionError;
use leptos::*;

#[component]
pub fn CalculatorPage() -> impl IntoView {
    let app_state = expect_app_state();
    let output = Signal::derive(move || {
        app_state
            .get()
            .session
            .last_entry()
            .and_then(|statement| Some(statement.output.clone()))
    });
    let trace = move || match output.get() {
        Some(result) => match result {
            Ok(_) => vec![],
            Err(error) => match error {
                SessionError::ParseNodeError(..) => vec![],
                SessionError::SerializeNodeError(..) => vec![],
                SessionError::EvaluateNodeError(evaluate_node_error) => {
                    evaluate_node_error.stack_trace()
                }
            },
        },
        None => vec![],
    };

    view! {
        <PageWithSidebar>
            <PageTitle>Calculator</PageTitle>
            <div class="flex flex-col gap-4">
                <InputField />
                <ul class="flex flex-col space-y-1">
                    {move || {
                        trace()
                            .iter()
                            .map(|trace| {
                                let spans = trace.spans();
                                view! {
                                    <li class="flex">
                                        <ul class="flex p-1 rounded-md border border-black">
                                            {spans
                                                .iter()
                                                .map(|span| {
                                                    view! {
                                                        <li class="flex p-1 space-x-2 bg-white rounded-sm border border-neutral-300">
                                                            <span>{span.start()}</span>
                                                            <span>{span.end()}</span>
                                                        </li>
                                                    }
                                                })
                                                .collect_view()}
                                        </ul>
                                    </li>
                                }
                            })
                            .collect_view()
                    }}
                </ul>
                <OutputField value=output />
            </div>
        </PageWithSidebar>
    }
}

#[component]
pub fn OutputField(
    value: Signal<Option<Result<String, SessionError>>>,
) -> impl IntoView {
    let output = move || match value.get() {
        None => (String::new(), false),
        Some(result) => match result {
            Ok(result) => (result, false),
            Err(error) => (error.to_string(), true),
        },
    };
    let content = move || output().0;
    let is_error = move || output().1;
    view! {
        <input
            data-testid="calculator-output"
            class="p-2 text-xl rounded-sm border outline-none focus-visible:border-black border-neutral-300"
            class:border-black=move || !is_error()
            class:border-red-500=move || is_error()
            class:bg-red-100=move || is_error()
            readonly
            value=content
        />
    }
}

use crate::{
    components::*,
    state::AppAction,
    utils::{expect_app_state, expect_dispatch},
};
use engine::SessionError;
use leptos::*;

#[component]
pub fn CalculatorPage() -> impl IntoView {
    let dispatch = expect_dispatch();
    let app_state = expect_app_state();
    let output = Signal::derive(move || {
        app_state
            .get()
            .session
            .last_entry()
            .and_then(|statement| Some(statement.output.clone()))
    });

    view! {
        <PageWithSidebar>
            <PageTitle>Calculator</PageTitle>
            <div class="flex flex-col gap-4">
                <input
                    data-testid="calculator-input"
                    class="border border-black p-2 text-xl rounded-sm"
                    type="text"
                    on:change=move |event| {
                        dispatch.call(AppAction::Evaluate(event_target_value(&event)))
                    }
                />

                <OutputField value=output/>
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
            class="border p-2 text-xl rounded-sm"
            class:border-black=move || !is_error()
            class:border-red-500=move || is_error()
            class:bg-red-100=move || is_error()
            readonly
            value=content
        />
    }
}

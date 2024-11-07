use crate::{
    components::*,
    pages::calculator::{
        input_field::InputField, use_last_output::use_last_output,
    },
};
use engine::SessionError;
use leptos::*;

#[component]
pub fn CalculatorPage() -> impl IntoView {
    let output = use_last_output();

    view! {
        <PageWithSidebar>
            <PageTitle>Calculator</PageTitle>
            <div class="flex flex-col gap-4">
                <InputField />
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

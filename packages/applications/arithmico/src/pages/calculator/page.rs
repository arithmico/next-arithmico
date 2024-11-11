use crate::{
    components::*,
    pages::calculator::{
        input_field::InputField, output::OutputField,
        use_last_output::use_last_output,
    },
};
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

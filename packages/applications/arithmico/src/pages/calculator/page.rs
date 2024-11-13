use crate::{
    components::*,
    pages::calculator::{
        input_field::InputField, output::OutputField,
        use_last_output::use_last_output,
    },
};
use leptos::*;
use translate::FormattedMessage;

#[component]
pub fn CalculatorPage() -> impl IntoView {
    let output = use_last_output();

    view! {
        <PageWithSidebar>
            <PageTitle>
                <FormattedMessage id="calculator.title" />
            </PageTitle>
            <div class="flex flex-col gap-4">
                <InputField />
                <OutputField value=output />
            </div>
        </PageWithSidebar>
    }
}

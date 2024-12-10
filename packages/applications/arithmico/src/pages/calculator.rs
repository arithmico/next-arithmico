use header::CalculatorHeader;
use input::CaluclatorInput;
use leptos::*;
use output::CalculatorOutput;

mod header;
mod input;
mod output;
mod use_error_trace;
mod use_last_output;

use use_last_output::use_last_output;

use crate::components::PageWithSidebar;

#[component]
pub fn CalculatorPage() -> impl IntoView {
    let output = use_last_output();

    view! {
        <PageWithSidebar>
            <CalculatorHeader />
            <div class="flex flex-col gap-4">
                <CaluclatorInput />
                <CalculatorOutput value=output />
            </div>
        </PageWithSidebar>
    }
}

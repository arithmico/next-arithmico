use calculator_header::CalculatorHeader;
use input::CaluclatorInput;
use leptos::prelude::*;
use output::CalculatorOutput;

mod calculator_header;
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
            <div class="calculator-layout">
                <CaluclatorInput />
                <CalculatorOutput value=output />
            </div>
        </PageWithSidebar>
    }
}

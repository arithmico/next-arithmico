use calculator_header::CalculatorHeader;
use calculator_input::CaluclatorInput;
use leptos::prelude::*;

mod calculator_header;
mod calculator_input;
mod use_error_trace;
mod use_last_output;

use use_last_output::use_last_output;

use crate::components::{CalculatorOutput, PageWithSidebar};

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

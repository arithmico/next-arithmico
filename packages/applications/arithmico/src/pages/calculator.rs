use calculator_header::CalculatorHeader;
use calculator_input::CaluclatorInput;
use leptos::prelude::*;
use web_state::WebState;

mod calculator_header;
mod calculator_input;
mod use_error_trace;

use crate::{
    components::{CalculatorOutput, PageWithSidebar},
    state::State,
};

#[component]
pub fn CalculatorPage() -> impl IntoView {
    let state = State::expect_state();
    let current_output = state.select(|state| state.current_output);

    view! {
        <PageWithSidebar class="calculator">
            <CalculatorHeader />
            <div class="calculator-layout">
                <CaluclatorInput />
                <CalculatorOutput value=current_output />
            </div>
        </PageWithSidebar>
    }
}

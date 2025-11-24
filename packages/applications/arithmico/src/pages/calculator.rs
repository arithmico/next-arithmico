use calculator_header::CalculatorHeader;
use calculator_input::CaluclatorInput;
use leptos::prelude::*;
use web_state::WebState;

mod calculator_header;
mod calculator_input;

use crate::{
    components::{CalculatorOutput, PageWithSidebar},
    state::State,
};

#[component]
pub fn CalculatorPage() -> impl IntoView {
    let state = State::expect_state();
    let current_output = state.select(|state| state.current_output);
    let context = state.select(|state| state.create_engine_context());

    view! {
        <PageWithSidebar class="calculator">
            <CalculatorHeader />
            <div class="calculator-layout">
                <CaluclatorInput />
                <CalculatorOutput
                    class="calculator-output-field"
                    value=current_output
                    context=context
                />
            </div>
        </PageWithSidebar>
    }
}

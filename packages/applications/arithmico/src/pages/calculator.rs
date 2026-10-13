use calculator_header::CalculatorHeader;
use calculator_input::CaluclatorInput;
use leptos::prelude::*;
use web_state::use_state;

mod calculator_header;
mod calculator_input;

use crate::{
    components::{CalculatorOutput, PageWithSidebar},
    state::State,
};

#[component]
pub fn CalculatorPage() -> impl IntoView {
    let state = use_state::<State>();
    let current_output = state.query(|state| state.current_output.clone());

    view! {
        <PageWithSidebar class="calculator">
            <CalculatorHeader />
            <div class="calculator-layout">
                <CaluclatorInput />
                <CalculatorOutput
                    class="calculator-output-field"
                    value=current_output
                />
            </div>
        </PageWithSidebar>
    }
}

use calculator_header::CalculatorHeader;
use calculator_input::CaluclatorInput;
use leptos::prelude::*;
use translate::FormattedMessage;
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
                <div>
                    <label id="calculator-input-label">
                        <FormattedMessage id="calculator.input.label" />
                    </label>
                    <CaluclatorInput attr:aria-labelledby="calculator-input-label" />
                </div>
                <div>
                    <label id="calculator-output-label">
                        <FormattedMessage id="calculator.output.label" />
                    </label>
                    <CalculatorOutput
                        attr:aria-labelledby="calculator-input-label"
                        attr:aria-atomic=true
                        attr:aria-live="polite"
                        class="calculator-output-field"
                        value=current_output
                    />
                </div>
            </div>
        </PageWithSidebar>
    }
}

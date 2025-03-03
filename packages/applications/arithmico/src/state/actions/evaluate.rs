use common::EvaluateNodeOptions;
use web_state::WebStateAction;

use crate::state::State;

pub struct EvaluateAction {
    input: String,
}

impl EvaluateAction {
    pub fn new(input: impl ToString) -> Self {
        Self {
            input: input.to_string(),
        }
    }
}

impl WebStateAction<State> for EvaluateAction {
    fn apply(&self, state: &mut State) {
        let options = EvaluateNodeOptions::new(
            (&state.settings.decimal_places).into(),
            state
                .settings
                .override_decimal_format
                .decimal_format()
                .cloned()
                .unwrap_or_else(|| (&state.settings.language).into()),
        );
        state.session.push(&self.input, &options);
    }
}

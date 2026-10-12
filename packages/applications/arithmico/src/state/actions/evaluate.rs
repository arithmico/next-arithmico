use web_state::StateMutation;

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

impl StateMutation<State> for EvaluateAction {
    fn apply(&self, state: &mut State) {
        let language = state.get_decimal_format();
        let angle_unit = state.settings.angle_unit;

        state.log.push(language, angle_unit, self.input.clone());
        state.current_output = state
            .log
            .entries()
            .last()
            .and_then(|statement| statement.output.clone());

        state.error_spans = state
            .current_output
            .as_ref()
            .and_then(|output| output.as_ref().err())
            .map(|output| output.get_spans())
            .unwrap_or_default()
    }
}

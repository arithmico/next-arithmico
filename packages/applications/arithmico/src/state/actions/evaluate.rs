use engine_protocol::Command;
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
        let correlation_id = state.worker_client.reserve_correlation_id();

        state.log.push(
            correlation_id,
            language,
            angle_unit,
            self.input.clone(),
        );

        state.worker_client.send_command(
            Some(correlation_id),
            Command::Evaluate {
                input: self.input.clone(),
                language,
                angle_unit,
            },
        );
    }
}

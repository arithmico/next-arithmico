use engine::SessionError;
use node::Node;
use web_state::StateMutation;

use crate::state::State;

#[derive(Debug, Clone)]
pub struct UpdateOutputAction {
    correlation_id: usize,
    output: Result<Node, SessionError>,
}

impl UpdateOutputAction {
    pub fn new(
        correlation_id: usize,
        output: Result<Node, SessionError>,
    ) -> Self {
        Self {
            correlation_id,
            output,
        }
    }
}

impl StateMutation<State> for UpdateOutputAction {
    fn apply(&self, state: &mut State) {
        state
            .log
            .set_output(self.correlation_id, self.output.clone());

        if state.log.last().map(|last| last.correlation_id)
            == Some(self.correlation_id)
        {
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
}

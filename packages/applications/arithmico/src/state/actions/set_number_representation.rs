use common::NumberRepresentation;
use web_state::StateMutation;

use crate::state::State;

pub struct SetNumberRepresentationAction {
    value: NumberRepresentation,
}

impl SetNumberRepresentationAction {
    pub fn new(value: impl Into<NumberRepresentation>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl StateMutation<State> for SetNumberRepresentationAction {
    fn apply(&self, state: &mut State) {
        state.settings.number_representation = self.value;
        state.settings.save();
    }
}

use common::AngleUnit;
use web_state::StateMutation;

use crate::state::State;

pub struct SetAngleUnitAction {
    value: AngleUnit,
}

impl SetAngleUnitAction {
    pub fn new(value: impl Into<AngleUnit>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl StateMutation<State> for SetAngleUnitAction {
    fn apply(&self, state: &mut State) {
        state.settings.angle_unit = self.value;
        state.settings.save();
    }
}

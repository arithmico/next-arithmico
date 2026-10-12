use web_state::StateMutation;

use crate::state::{State, theme::Theme};

pub struct SetThemeAction {
    value: Theme,
}

impl SetThemeAction {
    pub fn new(value: impl Into<Theme>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl StateMutation<State> for SetThemeAction {
    fn apply(&self, state: &mut State) {
        state.settings.theme = self.value;
        state.settings.save();
    }
}

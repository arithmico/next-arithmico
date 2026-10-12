use web_state::StateMutation;

use crate::state::{Settings, State};

pub struct ResetSettingsAction;

impl ResetSettingsAction {
    pub fn new() -> Self {
        Self
    }
}

impl StateMutation<State> for ResetSettingsAction {
    fn apply(&self, state: &mut State) {
        state.settings = Settings::default();
        state.settings.save();
    }
}

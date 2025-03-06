use web_state::WebStateAction;

use crate::state::{Settings, State};

pub struct ResetSettingsAction;

impl ResetSettingsAction {
    pub fn new() -> Self {
        Self
    }
}

impl WebStateAction<State> for ResetSettingsAction {
    fn apply(&self, state: &mut State) {
        state.settings = Settings::default();
        state.settings.save();
    }
}

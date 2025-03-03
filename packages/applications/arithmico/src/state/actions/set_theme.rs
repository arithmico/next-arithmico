use web_state::WebStateAction;

use crate::state::{theme::Theme, State};

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

impl WebStateAction<State> for SetThemeAction {
    fn apply(&self, state: &mut State) {
        state.settings.theme = self.value.clone();
        state.settings.save();
    }
}

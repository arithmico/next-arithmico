use web_state::WebStateAction;

use crate::state::{State, language::LanguageValue};

pub struct SetLanguageAction {
    value: LanguageValue,
}

impl SetLanguageAction {
    pub fn new(value: impl Into<LanguageValue>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl WebStateAction<State> for SetLanguageAction {
    fn apply(&self, state: &mut State) {
        state.settings.language = self.value.clone();
        state.settings.save();
    }
}

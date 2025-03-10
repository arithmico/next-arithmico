use ast::Language;
use web_state::WebStateAction;

use crate::state::State;

pub struct SetLanguageAction {
    value: Language,
}

impl SetLanguageAction {
    pub fn new(value: impl Into<Language>) -> Self {
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

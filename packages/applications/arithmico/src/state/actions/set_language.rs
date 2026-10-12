use web_state::StateMutation;

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

impl StateMutation<State> for SetLanguageAction {
    fn apply(&self, state: &mut State) {
        state.settings.language = self.value;
        state.settings.save();
    }
}

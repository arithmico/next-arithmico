use web_state::WebStateAction;

use crate::state::{output_format::OutputFormat, State};

pub struct SetOutputFormatAction {
    value: OutputFormat,
}

impl SetOutputFormatAction {
    pub fn new(value: impl Into<OutputFormat>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl WebStateAction<State> for SetOutputFormatAction {
    fn apply(&self, state: &mut State) {
        state.settings.output_format = self.value.clone();
        state.settings.save();
    }
}

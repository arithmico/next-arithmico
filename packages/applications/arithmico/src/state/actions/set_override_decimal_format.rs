use web_state::WebStateAction;

use crate::state::{State, override_decimal_format::OverrideDecimalFormat};

pub struct SetOverrideDecimalFormatAction {
    value: OverrideDecimalFormat,
}

impl SetOverrideDecimalFormatAction {
    pub fn new(value: impl Into<OverrideDecimalFormat>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl WebStateAction<State> for SetOverrideDecimalFormatAction {
    fn apply(&self, state: &mut State) {
        state.settings.override_decimal_format = self.value.clone();
        state.settings.save();
    }
}

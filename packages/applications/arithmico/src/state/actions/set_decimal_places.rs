use engine::DecimalPlaces;
use web_state::WebStateAction;

use crate::state::State;

pub struct SetDecimalPlacesAction {
    value: DecimalPlaces,
}

impl SetDecimalPlacesAction {
    pub fn new(value: DecimalPlaces) -> Self {
        Self { value }
    }
}

impl WebStateAction<State> for SetDecimalPlacesAction {
    fn apply(&self, state: &mut State) {
        state.settings.decimal_places = self.value.clone();
    }
}

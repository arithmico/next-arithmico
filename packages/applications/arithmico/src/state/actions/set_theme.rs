use web_state::WebStateAction;

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

impl WebStateAction<State> for SetThemeAction {
    fn apply(&self, state: &mut State) {
        state.settings.theme = match self.value {
            Theme::Default => {
                if web_sys::window()
                    .and_then(|window| {
                        window.match_media("(prefers-color-scheme: dark)")
                            .ok()
                            .flatten()
                    })
                    .map(|mq| mq.matches())
                    .unwrap_or(false)
                {
                    Theme::Dark
                } else {
                    Theme::Light
                }
            }
            _ => self.value.clone(),
        };
        state.settings.save();
    }
}

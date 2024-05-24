use std::rc::Rc;

use engine::{Language, Settings as EngineSettings};
use yew::Reducible;

use super::{app_state::Settings, AppState};

pub enum AppAction {
    SetInput(String),
    Evaluate,
    SetDecimalPlaces(u8),
    SetInterfaceLanguage(Language),
}

impl Reducible for AppState {
    type Action = AppAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let new_state: Rc<AppState> = match action {
            AppAction::SetInput(input) => Self {
                input,
                ..(*self).clone()
            }
            .into(),
            AppAction::Evaluate => {
                if self.input.len() == 0 {
                    return self;
                }
                Self {
                    session: self.session.push(
                        self.input.as_str(),
                        &EngineSettings::new(self.settings.decimal_places),
                    ),
                    ..(*self).clone()
                }
                .into()
            }
            AppAction::SetDecimalPlaces(decimal_places) => Self {
                settings: Settings {
                    decimal_places,
                    ..(*self).settings.clone()
                },
                ..(*self).clone()
            }
            .into(),
            AppAction::SetInterfaceLanguage(language) => Self {
                settings: Settings {
                    language,
                    ..(*self).settings.clone()
                },
                ..(*self).clone()
            }
            .into(),
        };
        new_state.settings.save().unwrap();
        new_state
    }
}

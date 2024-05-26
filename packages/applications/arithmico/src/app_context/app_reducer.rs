use std::rc::Rc;

use engine::Language;
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
                let settings = self.get_engine_settings();
                let new_session =
                    self.session.push(self.input.as_str(), &settings);
                let new_definitions = new_session.get_definitions(&settings);
                Self {
                    session: new_session,
                    definitions: new_definitions,
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

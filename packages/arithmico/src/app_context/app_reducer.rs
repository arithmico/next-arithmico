use std::rc::Rc;

use engine::Settings;
use yew::Reducible;

use super::AppState;

pub enum AppReducerAction {
    Evaluate(String),
}

impl Reducible for AppState {
    type Action = AppReducerAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            AppReducerAction::Evaluate(input) => Self {
                host_api: self.host_api.clone(),
                session: self.session.push(
                    input.as_str(),
                    &Settings::new(self.settings.decimal_places),
                ),
                settings: self.settings.clone(),
            }
            .into(),
        }
    }
}
